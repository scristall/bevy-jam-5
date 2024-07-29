use bevy::prelude::*;

mod behind;
mod bulletin_board;
mod desk;
mod door;
mod keypad_drawer;
mod lamp;
mod lock_drawer;
mod radio;
mod tv;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        behind::plugin,
        bulletin_board::plugin,
        keypad_drawer::plugin,
        lamp::plugin,
        lock_drawer::plugin,
        tv::plugin,
        desk::plugin,
        radio::plugin,
    ));
}
