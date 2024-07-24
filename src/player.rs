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
pub struct AmRadioFreq(pub u32);

fn setup(mut commands: Commands) {
    commands.spawn((Player, AmRadioFreq(600)));
}

fn update(
    mut radio_freqs: Query<&mut AmRadioFreq, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for mut radio_freq in &mut radio_freqs {
            if radio_freq.0 == 600 {
                radio_freq.0 = 720;
            } else {
                radio_freq.0 = 600;
            }
        }
    }
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
