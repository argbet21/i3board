mod input;

use bevy::prelude::*;
use input::keyboard_input::print_keyboard_event_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, print_keyboard_event_system)
        .run();
}
