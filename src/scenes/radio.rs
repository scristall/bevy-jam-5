use bevy::prelude::*;

use crate::{gamedata::SceneId, player::LoadScene};

fn load_scene(mut commands: Commands, mut load_scene: EventReader<LoadScene>) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Radio {}
    }
}

fn update() {}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
