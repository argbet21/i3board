use bevy::prelude::*;
use std::fmt::Display;

pub enum AudioMode {
    Pressed,
    Released,
}

impl Display for AudioMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pressed => write!(f, "{}", "pressed"),
            Self::Released => write!(f, "{}", "released"),
        }
    }
}

// `AudioSink` is used to control audio during playback.
// Bevy inserts this component onto entities when it begins playing an audio source. We use `AudioBundle` to trigger that to happen.
// If this component is removed from an entity, [and `insert`ed back again], that translates to the audio restarting.
pub fn play_audio(
    entity: Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mode: AudioMode,
) {
    // Adds the `AudioSink` component
    commands.entity(entity).insert(AudioBundle {
        source: asset_server.load(format!("audio/sfx/{}/key.ogg", mode)),
        ..default()
    });
    commands.entity(entity).remove::<AudioSink>(); // Remove the `AudioSink` component
}
