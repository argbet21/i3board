use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.550, 0.6, 0.685)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },));

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/keyboard/keyboard_final_raw.glb#Scene0"),
        ..default()
    });
}

pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}
