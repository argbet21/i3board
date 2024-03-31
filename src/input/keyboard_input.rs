use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn print_keyboard_event(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}

// This function performs 3 crucial tasks
// - It checks which key was pressed
// - It translates it accordingly
// - It plays a sound effect accordingly
pub fn handle_keyboard_event(
    key: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Name, &mut Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Press'n'Release start
    // An internal function to see if you *really* pressed a key IRL.
    if key.just_pressed(KeyCode::Digit0) {
        // And if so, iterate through all the entities in our scene
        for (name, mut transform) in query.iter_mut() {
            // And translate only the entity named "0" (downwards)
            // since we *really* only pressed the "0" key on our keyboard,
            // otherwise all entities would move, i.e: the whole keyboard.
            // Note: this assumes we've properly named your entities in Blender
            if name.as_str() == "0" {
                transform.translation.y -= 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_pressed/key.ogg"),
                    ..default()
                });
            }
        }
    }

    // An internal function to see if you *really* released a key IRL.
    if key.just_released(KeyCode::Digit0) {
        // And if so, iterate through all the entities in our scene.
        for (name, mut transform) in query.iter_mut() {
            // And translate only the entity named "0" (upwards)
            // since we *really* only pressed the "0" key on our keyboard,
            // otherwise all entities would move, i.e: the whole keyboard.
            // Note: this assumes we've properly named your entities in Blender
            if name.as_str() == "0" {
                transform.translation.y += 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_released/key.ogg"),
                    ..default()
                });
            }
        }
    }
    // Press'n'Release end

    // Press'n'Release start
    if key.just_pressed(KeyCode::Backspace) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "backspace" {
                transform.translation.y -= 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_pressed/backspace.ogg"),
                    ..default()
                });
            }
        }
    }

    if key.just_released(KeyCode::Backspace) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "backspace" {
                transform.translation.y += 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_released/backspace.ogg"),
                    ..default()
                });
            }
        }
    }
    // Press'n'Release end

    // Press'n'Release start
    if key.just_pressed(KeyCode::Enter) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "enter" {
                transform.translation.y -= 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_pressed/enter.ogg"),
                    ..default()
                });
            }
        }
    }

    if key.just_released(KeyCode::Enter) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "enter" {
                transform.translation.y += 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_released/enter.ogg"),
                    ..default()
                });
            }
        }
    }
    // Press'n'Release end

    // Press'n'Release start
    if key.just_pressed(KeyCode::Space) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "spacebar" {
                transform.translation.y -= 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_pressed/spacebar.ogg"),
                    ..default()
                });
            }
        }
    }

    if key.just_released(KeyCode::Space) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "spacebar" {
                transform.translation.y += 0.05;

                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/sfx/keyboard_released/spacebar.ogg"),
                    ..default()
                });
            }
        }
    }
    // Press'n'Release end
}
