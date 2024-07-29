use bevy::prelude::*;

use crate::gamedata::AmRadioFreq;

use std::fmt::Write;

#[derive(Component)]
struct FreqText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn update(
    player_radio_freqs: Query<&AmRadioFreq, Changed<AmRadioFreq>>,
    mut text: Query<&mut Text, With<FreqText>>,
) {
    for player_radio_freq in &player_radio_freqs {
        for mut text in &mut text {
            text.sections[0].value.clear();
            write!(text.sections[0].value, "{} kHz", player_radio_freq.0).unwrap();
        }
    }
}

pub fn background_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
