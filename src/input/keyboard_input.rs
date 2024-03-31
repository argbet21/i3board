use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn print_keyboard_event(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}

pub fn handle_event(key: Res<ButtonInput<KeyCode>>, mut query: Query<(&Name, &mut Transform)>) {
    if key.just_pressed(KeyCode::Digit0) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "0" {
                transform.translation.y -= 0.05;
            }
        }
    }

    if key.just_released(KeyCode::Digit0) {
        for (name, mut transform) in query.iter_mut() {
            if name.as_str() == "0" {
                transform.translation.y += 0.05;
            }
        }
    }
}
