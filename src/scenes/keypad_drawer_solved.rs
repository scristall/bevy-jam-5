use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableLabel, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, Player, SceneItem, SceneState},
};

#[derive(Component)]
struct SurgeProtector;

fn load_scene(mut commands: Commands, mut load_scene: EventReader<LoadScene>) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::KeypadDrawerSolved {
            commands.spawn((
                ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                    Vec2::new(-121.0, 305.0),
                    300.0,
                    300.0,
                )),
                SurgeProtector,
                ClickableLabel("Surge Protector"),
                SceneItem(SceneId::KeypadDrawerSolved),
            ));
        }
    }
}

fn update(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    clickables: Query<&ClickableShape, With<SurgeProtector>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for clickable in clickables.iter() {
        if clickable.contains(mouse_pos.0) {
            player.has_surge_protector = true;
            player.scene = SceneState::ForceTransition(
                SceneId::KeypadDrawerSolved,
                SceneId::KeypadDrawerEmpty,
            );
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
