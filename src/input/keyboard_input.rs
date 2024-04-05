use crate::{
    audio::audio::play_audio,
    parser::parser::{Key, KeyCodeWrapper, Keys},
};
use bevy::prelude::*;
use std::collections::HashMap;

pub fn handle_keyboard_event(
    // Get immutable access to the `ButtonInput` resource. This resource describes a "press-able" input of type `T`.
    key: Res<ButtonInput<KeyCode>>,
    // Get all the entities marked with a `Key` component.
    mut query: Query<(Entity, &Key, &mut Transform)>,

    // These 2 parameters are used for the `play_audio` helper function.
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

            for (entity, key, mut transform) in query.iter_mut() {
                if key.name == pressed_key.unwrap().as_str() {
                    transform.translation.y -= 0.05;
                    play_audio(entity, &mut commands, &asset_server);
                }
            }
        }

        // `ButtonInput::just_released` returns true one frame after a release event if `input` has just been released.
        if key.just_released(keycode) {
            let released_key = hashmap.get(&(index as u8));

            for (entity, key, mut transform) in query.iter_mut() {
                if key.name == released_key.unwrap().as_str() {
                    transform.translation.y += 0.05;
                    play_audio(entity, &mut commands, &asset_server);
                }
            }
        }
    }
}
