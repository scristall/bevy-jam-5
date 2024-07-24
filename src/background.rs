use bevy::prelude::*;

use crate::player::{AmRadioFreq, Player};

#[derive(Component)]
struct FreqText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/bg.png"),
        ..Default::default()
    });
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "600 kHz",
                TextStyle {
                    color: Color::BLACK,
                    font: asset_server.load("fonts/Belleza-Regular.ttf"),
                    font_size: 80.0,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_rotation(Quat::from_rotation_z(-0.04))
                .with_translation(Vec3::new(590.0, -185.0, 1.0)),
            ..default()
        },
        FreqText,
    ));
}

fn update(
    player_radio_freqs: Query<&AmRadioFreq, (With<Player>, Changed<AmRadioFreq>)>,
    mut text: Query<&mut Text, With<FreqText>>,
) {
    for player_radio_freq in &player_radio_freqs {
        for mut text in &mut text {
            text.sections[0].value = format!("{} kHz", player_radio_freq.0);
        }
    }
}

pub fn background_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
