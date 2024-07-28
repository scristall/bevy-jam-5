use crate::camera::{HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};
use crate::components::{Keyboard, UpdateSet};
use crate::gamedata::{debug_text_style, RenderLayer, SceneId, ScenePlayerControl};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Anchor;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub enum SceneState {
    Active(SceneId),
    Transitioning(SceneId, SceneId, u8),
}

#[derive(Component)]
pub struct Background;

#[derive(Resource)]
pub struct Player {
    scene: SceneState,
}

impl Player {
    fn new() -> Self {
        Self {
            scene: SceneState::Active(SceneId::Desk),
        }
    }
}

#[derive(Component)]
struct DebugSceneText;

fn scene_transition_system(mut player: ResMut<Player>) {
    const TICKS_PER_TRANSITION: u8 = 15;

    match &mut player.scene {
        SceneState::Transitioning(_, next, tick) => {
            *tick += 1;
            if *tick == TICKS_PER_TRANSITION {
                player.scene = SceneState::Active(*next);
            }
        }
        _ => (),
    }
}

fn keyboard_input_system(keyboard: Keyboard, mut player: ResMut<Player>) {
    const SCENE_TRANSITION_CONTROLS: [ScenePlayerControl; 3] = [
        ScenePlayerControl::TransitionSceneLeft,
        ScenePlayerControl::TransitionSceneRight,
        ScenePlayerControl::TransitionSceneBehind,
    ];

    if let SceneState::Active(scene) = player.scene {
        for (control, key) in SCENE_TRANSITION_CONTROLS
            .iter()
            .copied()
            .map(|control| (control, ScenePlayerControl::key_code(control)))
        {
            if keyboard.just_pressed(key) {
                let next = scene.next_scene(control);
                if let Some(next) = next {
                    player.scene = SceneState::Transitioning(scene, next, 0);
                    return;
                }
            }
        }
    }
}

fn render_bg_system(
    player: Res<Player>,
    mut bg: Query<&mut Handle<Image>, With<Background>>,
    asset_server: Res<AssetServer>,
) {
    let mut bg = bg.get_single_mut().unwrap();
    match player.scene {
        SceneState::Active(id) => *bg = asset_server.load(id.asset_path()),
        SceneState::Transitioning(_prev, _next, _tick) => (), //TODO: do some mixing
    }
}

fn debug_update(player: Res<Player>, mut debug_text: Query<&mut Text, With<DebugSceneText>>) {
    for mut text in &mut debug_text {
        text.sections[1].value.clear();

        write!(&mut text.sections[1].value, "{:?} ", player.scene).unwrap();
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
        RenderLayers::layer(RenderLayer::DebugText as usize),
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle::default(),
        Background,
        RenderLayers::layer(RenderLayer::Background as usize),
    ));
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Player::new());
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            scene_transition_system,
            keyboard_input_system,
            render_bg_system,
        )
            .chain()
            .in_set(UpdateSet::PreScene),
    );
    if cfg!(feature = "debug_state") {
        app.add_systems(Startup, debug_setup);
        app.add_systems(Update, (debug_update).in_set(UpdateSet::Debug));
    }
}
