use bevy::{prelude::*, render::view::RenderLayers};

use super::{tv_ending::TvPlayerKilled, tv_player::{TvControlled, TvPlayer}};

#[derive(Component)]
pub struct TvMonster;

const MONSTER_SPEED: f32 = 0.01;
const KILL_DISTANCE: f32 = 20.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/tv/monster.png"),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..Default::default()
        },
        RenderLayers::layer(1),
        TvControlled {
            puzzle_pos: 0,
        },
        TvMonster,
    ));
}

fn update(
    tv_player: Query<&Transform, With<TvPlayer>>,
    mut tv_monster: Query<&mut Transform, (With<TvMonster>, Without<TvPlayer>)>,
    mut player_killed: EventWriter<TvPlayerKilled>,
) {
    if let Ok(p_tform) = tv_player.get_single() {
        if let Ok(mut m_tform) = tv_monster.get_single_mut() {
            let diff = (p_tform.translation - m_tform.translation).normalize();
            *m_tform = m_tform.with_translation(m_tform.translation + diff * MONSTER_SPEED);

            if m_tform.translation.distance(p_tform.translation) < KILL_DISTANCE {
                player_killed.send(TvPlayerKilled);
            }
        }
    }
}

pub fn tv_monster_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
