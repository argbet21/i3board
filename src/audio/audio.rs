use bevy::prelude::*;

// `AudioSink` is used to control audio during playback.
// Bevy inserts this component onto entities when it begins playing an audio source. We use `AudioBundle` to trigger that to happen.
// If this component is removed from an entity, [and `insert`ed back again], that translates to the audio restarting.
pub fn play_audio(entity: Entity, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Adds the `AudioSink` component
    commands.entity(entity).insert(AudioBundle {
        source: asset_server.load("audio/sfx/pressed/key.ogg"),
        ..default()
    });
    commands.entity(entity).remove::<AudioSink>(); // Remove the `AudioSink` component
}
