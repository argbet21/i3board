use bevy::{input::keyboard::KeyboardInput, prelude::*};

// This system prints out all keyboard events as they come in
pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}
