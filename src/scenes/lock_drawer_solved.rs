use bevy::prelude::*;


use crate::{
    components::{ClickableArea, ClickableLabel, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, Player, SceneItem, SceneState},
};

#[derive(Component)]
struct MorseCodeTranslator;

fn load_scene(mut commands: Commands, mut load_scene: EventReader<LoadScene>) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::LockDrawerSolved {
            commands.spawn((
                ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                    Vec2::new(-100.0, 100.0),
                    800.0,
                    400.0,
                )),
                MorseCodeTranslator,
                ClickableLabel("Radio Module"),
                SceneItem(SceneId::LockDrawerSolved),
            ));
        }
    }
}

fn update(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    clickables: Query<&ClickableShape, With<MorseCodeTranslator>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for clickable in clickables.iter() {
        if clickable.contains(mouse_pos.0) {
            player.has_morse_code_translator = true;
            player.scene = SceneState::ForceTransition(
                SceneId::LockDrawerSolved,
                SceneId::LockDrawerEmpty,
            );
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
