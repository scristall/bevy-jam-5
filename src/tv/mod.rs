use bevy::prelude::*;

mod screen;
mod tv_ending;
mod tv_monster;
mod tv_player;
mod whirlpool;

pub fn tv_plugin(app: &mut App) {
    app.add_plugins((
        screen::screen_plugin,
        tv_player::tv_player_plugin,
        tv_monster::tv_monster_plugin,
        tv_ending::tv_ending_plugin,
        whirlpool::whirlpool_plugin
    ));
}
