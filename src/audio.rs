use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::player::{AmRadioFreq, Player};

#[derive(Component)]
pub struct RadioStation {
    handle: Handle<AudioInstance>,
    frequency: u32, // in kHz
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct WhiteNoise(Handle<AudioInstance>);

fn update(
    radio_freqs: Query<&AmRadioFreq, (With<Player>, Changed<AmRadioFreq>)>,
    radio_stations: Query<&RadioStation>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    time: Res<Time>,
) {
    for radio_freq in &radio_freqs {
        for radio_station in &radio_stations {
            if let Some(instance) = audio_instances.get_mut(&radio_station.handle) {
                if radio_freq.0 == radio_station.frequency {
                    instance.seek_to(time.elapsed_seconds_f64());
                    instance.resume(AudioTween::default());
                } else {
                    instance.pause(AudioTween::default());
                }
            }
        }
    }
}

fn setup(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    commands.insert_resource(WhiteNoise(
        audio
            .play(asset_server.load("audio/whitenoise.ogg"))
            .looped()
            .handle(),
    ));
    commands.spawn(RadioStation {
        handle: audio
            .play(asset_server.load("audio/number-station.ogg"))
            .looped()
            .paused()
            .handle(),
        frequency: 720,
    });
}

pub fn audio_plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
