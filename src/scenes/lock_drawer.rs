use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableLabel, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, Player, SceneItem, SceneState},
};

#[derive(Component)]
struct Key;

fn load_scene(mut commands: Commands, mut load_scene: EventReader<LoadScene>) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::LockDrawer {
            commands.spawn((
                ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                    Vec2::new(570.0, 357.0),
                    150.0,
                    150.0,
                )),
                ClickableLabel("Lock"),
                Key,
                SceneItem(SceneId::LockDrawer),
            ));
        }
    }
}

fn update(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    key: Query<(Entity, &ClickableShape), With<Key>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for key in key.iter() {
        if key.1.contains(mouse_pos.0) {
            if player.has_key {
                player.opened_key_drawer = true;
                player.scene =
                    SceneState::ForceTransition(SceneId::LockDrawer, SceneId::LockDrawerSolved);
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
