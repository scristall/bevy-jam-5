use bevy::prelude::*;

mod behind;
mod bulletin_board;
mod desk;
mod door;
mod keypad_drawer;
mod keypad_drawer_solved;
mod lamp;
mod lock_drawer;
mod lock_drawer_solved;
mod radio;
mod phone;
mod tv;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        behind::plugin,
        bulletin_board::plugin,
        keypad_drawer::plugin,
        keypad_drawer_solved::plugin,
        lamp::plugin,
        lock_drawer::plugin,
        lock_drawer_solved::plugin,
        tv::plugin,
        desk::plugin,
        radio::plugin,
        phone::plugin
    ));
}
