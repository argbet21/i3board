mod input;

use bevy::prelude::*;
use input::keyboard_input::{print_keyboard_event_system, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.46, 0.46, 0.46)))
        .add_systems(Startup, setup)
        .add_systems(Update, print_keyboard_event_system)
        .run();
}
