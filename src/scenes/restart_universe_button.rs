use bevy::prelude::*;

use crate::{
    components::{ClickableShape, Rectangle},
    gamedata::SceneId,
    player::LoadScene,
};

fn load_scene(mut commands: Commands, mut load_scene: EventReader<LoadScene>) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Behind {
            commands.spawn((
                SpriteBundle {
                    ..Default::default()
                },
                ClickableShape::Rectangle(Rectangle {
                    top_left: Vec2::new(50.0, 50.0),
                    bottom_right: Vec2::new(100.0, 100.0),
                }),
            ));
        }
    }
}

fn update() {}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
