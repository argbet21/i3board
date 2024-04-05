use crate::parser::parser::{Key, KeyCodeWrapper, Keys};
use bevy::prelude::*;
use std::collections::HashMap;

// This function performs 4 crucial tasks.
// - Parsing:
//   Keys have predetermined values in Blender. This function maps those to sequential ID's in Rust.
//   We do this to get a range (0..104) of keys that we can iterate over.
//   The alternative to a single contiguous range is a set of non-contiguous ranges (e.g., { 0..=9, a..=z, ..etc }).
//   This causes a number of issues as it adds:
//      1. Complexity: How would an implementation of keys that do not implement the `Range` trait look like? (e.g., f1..=f12, arrow keys, ..etc).
//      2. Code Duplication: The core logic for every key is the same. Having multiple ranges violates the DRY principle.
//
// - It iterates through a built-in `KeyCode` range and checks which key was pressed.
// - It translates it accordingly
// - It plays a sound effect accordingly
pub fn handle_keyboard_event(
    // Get immutable access to the `ButtonInput` resource. This resource describes a "press-able" input of type `T`.
    key: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Key, &mut Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut hashmap: HashMap<u8, String> = HashMap::with_capacity(104);
    for (index, key) in Keys::into_iter().enumerate() {
        hashmap.insert(index as u8, String::from(key));
    }

    // Iterate over all `KeyCode`'s, checking whether any has been pressed and/or released.
    for (index, keycode) in KeyCodeWrapper::into_iter().enumerate() {
        // `ButtonInput::just_pressed` will return true one frame after a press event if `input` has just been pressed.
        if key.just_pressed(keycode) {
            // Get the entity corresponding to the pressed `KeyCode`.
            // Note that this works because the iterators - `KeyCodeWrapper::into_iter()` and `Keys::into_iter()` are in-sync.
            // See `crate::parser::parser.rs` for more details.
            let pressed_key = hashmap.get(&(index as u8));

            // And if so, iterate through all the keys in our scene
            for (key, mut transform) in query.iter_mut() {
                // And translate only the `pressed_key` (downwards)
                // since we *really* only pressed the `pressed_key` key on our keyboard,
                // otherwise all entities would move, i.e: the whole keyboard.
                if key.name == pressed_key.unwrap().as_str() {
                    transform.translation.y -= 0.05;

                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/sfx/released/key.ogg"),
                        ..default()
                    });
                }
            }
        }

        // `ButtonInput::just_released` returns true one frame after a release event if `input` has just been released.
        if key.just_released(keycode) {
            let released_key = hashmap.get(&(index as u8));

            // And if so, iterate through all the keys in our scene.
            for (key, mut transform) in query.iter_mut() {
                // And translate only the `pressed_key` (upwards)
                // since we *really* only pressed the `pressed_key` key on our keyboard,
                // otherwise all entities would move, i.e: the whole keyboard.
                if key.name == released_key.unwrap().as_str() {
                    transform.translation.y += 0.05;

                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/sfx/released/key.ogg"),
                        ..default()
                    });
                }
            }
        }
    }
}
