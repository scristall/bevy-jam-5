use crate::{
    components::UpdateSet,
    player::{Player, ResetUniverse},
    right_speaker::RightSpeakerDestroyed,
};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::gamedata::{AmRadioFreq, PresetAmRadioFreq};

// Outer bandwidth defines when the channel starts to be heard
// Inner bandwidth defines when the channel is at max volume
const STATION_OUTER_BANDWIDTH_DELTA: i32 = 30;
const STATION_INNER_BANDWIDTH_DELTA: i32 = 10;

#[derive(Component)]
pub struct RadioStation {
    handle: Handle<AudioInstance>,
    frequency: AmRadioFreq,
    playing: bool,
}

#[derive(Component)]
pub struct RadioAudio;

#[derive(Resource)]
#[allow(dead_code)]
pub struct WhiteNoise(Handle<AudioInstance>);

impl RadioStation {
    fn freq_in_band(&self, freq: i32) -> bool {
        (freq - self.frequency.0).abs() <= STATION_OUTER_BANDWIDTH_DELTA
    }

    fn freq_to_volume(&self, freq: i32) -> f64 {
        if !self.freq_in_band(freq) {
            return 0.0;
        }

        let delta = (freq - self.frequency.0).abs();
        if delta <= STATION_INNER_BANDWIDTH_DELTA {
            return 1.0;
        }

        let volume_sqrt = 1.0
            - ((delta - STATION_INNER_BANDWIDTH_DELTA) as f64
                / (STATION_OUTER_BANDWIDTH_DELTA - STATION_INNER_BANDWIDTH_DELTA) as f64);

        volume_sqrt * volume_sqrt
    }
}

fn on_right_speaker_destroyed(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    radio_stations: Query<&RadioStation>,
    whitenoise: Res<WhiteNoise>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut right_speaker_destroyed: EventReader<RightSpeakerDestroyed>,
) {
    for _ in right_speaker_destroyed.read() {
        audio.play(asset_server.load("audio/explosion.ogg"));

        for radio_station in radio_stations.iter() {
            if let Some(instance) = audio_instances.get_mut(&radio_station.handle) {
                instance.set_panning(0.0, AudioTween::default());
            }
        }
        if let Some(instance) = audio_instances.get_mut(&whitenoise.0) {
            instance.set_panning(0.0, AudioTween::default());
        }
    }
}

fn on_reset_universe(
    radio_stations: Query<&RadioStation>,
    whitenoise: Res<WhiteNoise>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut reset_universe: EventReader<ResetUniverse>,
) {
    for _ in reset_universe.read() {
        for radio_station in radio_stations.iter() {
            if let Some(instance) = audio_instances.get_mut(&radio_station.handle) {
                instance.set_panning(0.5, AudioTween::default());
            }
        }
        if let Some(instance) = audio_instances.get_mut(&whitenoise.0) {
            instance.set_panning(0.5, AudioTween::default());
        }
    }
}

fn update(
    radio_freqs: Query<&AmRadioFreq, (Changed<AmRadioFreq>, With<RadioAudio>)>,
    whitenoise: Res<WhiteNoise>,
    mut radio_stations: Query<&mut RadioStation>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    time: Res<Time>,
    player: Res<Player>,
) {
    if radio_freqs.is_empty() {
        return;
    }

    let mut whitenoise_volume = 1.0;
    for radio_freq in &radio_freqs {
        for mut radio_station in &mut radio_stations {
            if let Some(instance) = audio_instances.get_mut(&radio_station.handle) {
                if radio_station.freq_in_band(radio_freq.0) {
                    if !radio_station.playing {
                        instance.seek_to(time.elapsed_seconds_f64());
                        instance.resume(AudioTween::default());
                        radio_station.playing = true;
                    }

                    let station_volume = radio_station.freq_to_volume(radio_freq.0);
                    instance.set_volume(station_volume, AudioTween::default());

                    if player.right_speaker_broken {
                        instance.set_panning(0.0, AudioTween::default());
                    } else {
                        instance.set_panning(0.5, AudioTween::default());
                    }

                    whitenoise_volume = 0.5 + 0.5 * (1.0 - station_volume);
                } else {
                    instance.pause(AudioTween::default());
                    radio_station.playing = false;
                }
            }
        }
    }
    if let Some(instance) = audio_instances.get_mut(&whitenoise.0) {
        instance.set_volume(whitenoise_volume, AudioTween::default());

        if player.right_speaker_broken {
            instance.set_panning(0.0, AudioTween::default());
        } else {
            instance.set_panning(0.5, AudioTween::default());
        }
    }
}

fn setup(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    commands.spawn((AmRadioFreq(600), RadioAudio));

    commands.insert_resource(WhiteNoise(
        audio
            .play(asset_server.load("audio/whitenoise.ogg"))
            .looped()
            .handle(),
    ));
    commands.spawn(RadioStation {
        handle: audio
            .play(asset_server.load("audio/morse-code.ogg"))
            .looped()
            .paused()
            .handle(),
        frequency: PresetAmRadioFreq::Morse.into(),
        playing: false,
    });
    commands.spawn(RadioStation {
        handle: audio
            .play(asset_server.load("audio/number-station.ogg"))
            .looped()
            .paused()
            .handle(),
        frequency: PresetAmRadioFreq::Numbers.into(),
        playing: false,
    });
    commands.spawn(RadioStation {
        handle: audio
            .play(asset_server.load("audio/song.ogg"))
            .looped()
            .paused()
            .handle(),
        frequency: PresetAmRadioFreq::Music.into(),
        playing: false,
    });
    commands.spawn(RadioStation {
        handle: audio
            .play(asset_server.load("audio/news.ogg"))
            .looped()
            .paused()
            .handle(),
        frequency: PresetAmRadioFreq::News.into(),
        playing: false,
    });
}

pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (update, on_right_speaker_destroyed, on_reset_universe).in_set(UpdateSet::Scene),
    );
}
