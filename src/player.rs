use bevy::prelude::*;

#[allow(dead_code)]
pub enum Area {
    Default,
    AmRadio,
    Tv,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AmRadioFreq(pub i32);

fn setup(mut commands: Commands) {
    commands.spawn((Player, AmRadioFreq(650)));
}

fn update(
    mut radio_freqs: Query<&mut AmRadioFreq, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut radio_freq = radio_freqs.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        radio_freq.0 = 700;
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        radio_freq.0 -= 1;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        radio_freq.0 += 1;
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
