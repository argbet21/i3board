use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        // The camera is at this point: (0.0, 3.625, 0.0)
        transform: Transform::from_xyz(0.0, 3.625, 0.0)
            // And it's looking at this point: (0.0, 0.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::ANTIQUE_WHITE,
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(2.0, 8.0, 2.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)
                * Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}

pub fn spawn_keyboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/keyboard/keyboard.glb#Scene0"),
        transform: Transform {
            rotation: Quat::from_rotation_y(PI / 2.0),
            ..default()
        },
        ..default()
    });
}
