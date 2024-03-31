mod input;
mod setup;

use bevy::prelude::*;
use input::keyboard_input::{handle_event, print_keyboard_event};
use setup::setup::{spawn_camera, spawn_keyboard, spawn_light};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.46, 0.46, 0.46))) // To have the canvas blend with our terminal
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_light)
        .add_systems(Startup, spawn_keyboard)
        .add_systems(Update, print_keyboard_event)
        .add_systems(Update, handle_event)
        .run();
}
