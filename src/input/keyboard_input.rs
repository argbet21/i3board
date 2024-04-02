use bevy::prelude::*;
use std::collections::HashMap;

use super::parse::{KeyCodeWrapper, Keys};

// This function performs 4 crucial tasks.
// - Parsing:
//   Keys have predetermined values in Blender. This function maps those to sequential ID's in Rust.
//   We do this to get a range (0..104) of keys that we can iterate over.
//   The alternative to a single contiguous range is a set of non-contiguous ranges (e.g., { 0..=9, a..=z, ..etc }).
//   This causes a number of issues as it adds:
//      1. Complexity: How would an implementation of keys that do not implement the `Range` trait look like? (e.g., f1..=f12, arrow keys, ..etc).
//      2. Code Duplication: The core logic for every key is the same. Having multiple ranges violates the DRY principle.
//
// - It iterates through a built in `KeyCode` range and checks which key was pressed.
// - It translates it accordingly
// - It plays a sound effect accordingly
pub fn handle_keyboard_event(
    key: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Name, &mut Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut hashmap: HashMap<u8, String> = HashMap::with_capacity(104);
    for (index, key) in Keys::into_iter().enumerate() {
        hashmap.insert(index as u8, String::from(key));
    }

    for (index, keycode) in KeyCodeWrapper::into_iter().enumerate() {
        // An internal function to see if you *really* pressed a key IRL.
        if key.just_pressed(keycode) {
            let pressed_key = hashmap.get(&(index as u8));

            // And if so, iterate through all the keys in our scene
            for (name, mut transform) in query.iter_mut() {
                // And translate only the `pressed_key` (downwards)
                // since we *really* only pressed the `pressed_key` key on our keyboard,
                // otherwise all entities would move, i.e: the whole keyboard.
                if name.as_str() == pressed_key.unwrap().as_str() {
                    transform.translation.y -= 0.05;

                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/sfx/pressed/key.ogg"),
                        ..default()
                    });
                }
            }
        }

        // An internal function to see if you *really* released a key IRL.
        if key.just_released(keycode) {
            let released_key = hashmap.get(&(index as u8));

            // And if so, iterate through all the keys in our scene.
            for (name, mut transform) in query.iter_mut() {
                // And translate only the `pressed_key` (upwards)
                // since we *really* only pressed the `pressed_key` key on our keyboard,
                // otherwise all entities would move, i.e: the whole keyboard.
                if name.as_str() == released_key.unwrap().as_str() {
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
