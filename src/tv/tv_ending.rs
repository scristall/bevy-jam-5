use bevy::{prelude::*, render::view::RenderLayers};
use bevy_kira_audio::prelude::*;

use super::{tv_monster::TvMonster, tv_player::TvPlayer, TvComponent};

#[derive(Event)]
pub struct TvPlayerKilled;

fn update(
    mut commands: Commands,
    tv_entities: Query<Entity, Or<(With<TvPlayer>, With<TvMonster>)>>,
    mut player_killed: EventReader<TvPlayerKilled>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for _ in player_killed.read() {
        for e in tv_entities.iter() {
            commands.entity(e).despawn();
            audio
                .play(asset_server.load("audio/tv/player_die_monster.ogg"))
                .with_volume(0.3);
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/tv/doomed.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 3.0),
                    ..Default::default()
                },
                RenderLayers::layer(1),
                TvComponent,
            ));
        }
    }
}

pub fn tv_ending_plugin(app: &mut App) {
    app.add_event::<TvPlayerKilled>();
    app.add_systems(Update, update);
}
