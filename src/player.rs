use crate::camera::{HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};
use crate::components::{Keyboard, UpdateSet};
use crate::gamedata::{debug_text_style, SceneId, ScenePlayerControl};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fmt::Write;

#[derive(Resource)]
pub struct Player {
    active_scene: SceneId,
    transitioning_scenes: bool,
}

#[derive(Component)]
struct DebugSceneText;

fn scene_transition_update(keyboard: Keyboard, mut player: ResMut<Player>) {
    const SCENE_TRANSITION_CONTROLS: [ScenePlayerControl; 3] = [
        ScenePlayerControl::TransitionSceneLeft,
        ScenePlayerControl::TransitionSceneRight,
        ScenePlayerControl::TransitionSceneBehind,
    ];
    for (control, key) in SCENE_TRANSITION_CONTROLS
        .iter()
        .copied()
        .map(|control| (control, ScenePlayerControl::key_code(control)))
    {
        if keyboard.just_pressed(key) {
            let next_scene = player.active_scene.next_scene(control);
            if let Some(scene) = next_scene {
                // TODO: visually transition between the scenes
                // TODO: maybe lock out actions for N frames as the visual transition occurs?
                player.active_scene = scene;
            }
            return;
        }
    }
}

fn debug_update(player: Res<Player>, mut debug_text: Query<&mut Text, With<DebugSceneText>>) {
    for mut text in &mut debug_text {
        text.sections[1].value.clear();

        write!(&mut text.sections[1].value, "{:?} ", player.active_scene).unwrap();
    }
}

fn debug_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = debug_text_style(&asset_server);
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Active Scene: ", style.clone()),
                TextSection::from_style(style.clone()),
            ])
            .with_justify(JustifyText::Left),
            text_anchor: Anchor::CenterLeft,
            transform: Transform::from_translation(Vec3::new(
                -HORIZONTAL_RESOLUTION / 2.0 * 9.0 / 10.0,
                VERTICAL_RESOLUTION / 2.0 * 9.0 / 10.0,
                0.0,
            )),
            ..default()
        },
        DebugSceneText,
    ));
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Player {
        active_scene: SceneId::Desk,
        transitioning_scenes: false,
    });
    app.add_systems(
        Update,
        (scene_transition_update).in_set(UpdateSet::PreScene),
    );
    if cfg!(feature = "debug_state") {
        app.add_systems(Startup, debug_setup);
        app.add_systems(Update, (debug_update).in_set(UpdateSet::Debug));
    }
}
