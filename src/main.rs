mod audio;
mod input;
mod parser;
mod setup;

use bevy::prelude::*;
use input::keyboard_input::handle_keyboard_event;
use parser::parser::mark_entities;
use setup::setup::{spawn_camera, spawn_keyboard, spawn_light};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.46, 0.46, 0.46)))
        .add_systems(Startup, (spawn_camera, spawn_light, spawn_keyboard))
        // `mark_entities` should ideally be scheduled at `Startup`.
        // However, after testing and observation, this isn't possible as it seems that Bevy runs the `mark_entities` system *before* all our Blender assets have fully initialized.
        // Thus, a one-time run of `mark_entities` at `Startup` before asset initialization results in no entity marking occurring (which other systems *rely* on).
        // In order to combat this effect, we have `mark_entities` under the `Update` schedule and have mechanisms in-place (in its implementation) to *not* have the system constantly mark entities every frame.
        // See `crate::parser::parser::mark_entities` for more details.
        .add_systems(Update, (mark_entities, handle_keyboard_event).chain())
        .run();
}
