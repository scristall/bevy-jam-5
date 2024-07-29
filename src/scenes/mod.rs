use bevy::prelude::*;

mod desk;
mod restart_universe_button;

pub fn plugin(app: &mut App) {
    app.add_plugins((desk::plugin, restart_universe_button::plugin));
}
