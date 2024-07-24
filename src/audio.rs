use bevy::{audio::PlaybackMode, prelude::*};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/whitenoise.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
        ..default()
    });
}

pub fn audio_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
