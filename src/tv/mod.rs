use bevy::prelude::*;

mod screen;
mod tv_ending;
mod tv_monster;
mod tv_player;
mod whirlpool;

pub use screen::TvScreenMaterial;

use crate::{components::UpdateSet, player::ResetUniverse};

#[derive(Component)]
pub struct TvComponent;

#[derive(Event)]
pub struct TvStart;

fn reset_tv(
    mut commands: Commands,
    tv_components: Query<Entity, With<TvComponent>>,
    mut reset_universe: EventReader<ResetUniverse>,
    mut tv_start: EventWriter<TvStart>,
) {
    for _ in reset_universe.read() {
        for component in tv_components.iter() {
            commands.entity(component).despawn();
        }
        tv_start.send(TvStart);
    }
}

pub fn tv_plugin(app: &mut App) {
    app.add_event::<TvStart>();
    app.add_systems(Update, reset_tv.in_set(UpdateSet::PostScene));
    app.add_plugins((
        screen::screen_plugin,
        tv_player::tv_player_plugin,
        tv_monster::tv_monster_plugin,
        tv_ending::tv_ending_plugin,
        whirlpool::whirlpool_plugin,
    ));
}
