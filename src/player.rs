use crate::camera::{HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};
use crate::components::{Keyboard, UpdateSet};
use crate::gamedata::{debug_text_style, RenderLayer, SceneId, ScenePlayerControl};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fmt::Write;

const TICKS_PER_TRANSITION: u8 = 15;

#[derive(Debug, Clone)]
pub enum SceneState {
    Active(SceneId),
    Transitioning(SceneId, SceneId, u8),
}

#[derive(Event)]
pub struct LoadScene(pub SceneId);

#[derive(Event)]
pub struct UnloadScene(pub SceneId);

#[derive(Event)]
pub struct ResetUniverse;

#[derive(Event)]
pub struct TransitionToScene(pub SceneId);

#[derive(Component)]
pub struct SceneItem(pub SceneId);

#[derive(Component)]
pub struct Background0;

#[derive(Component)]
pub struct Background1;

pub enum LightbulbColor {
    Green,
    Red,
}

#[derive(Resource)]
pub struct Player {
    pub scene: SceneState,
    pub behind_puzzle_state: [usize; 6],
    pub keypad_drawer_puzzle_state: [usize; 3],
    pub lightbulb_unlock: Option<LightbulbColor>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            scene: SceneState::Active(SceneId::Desk),
            behind_puzzle_state: [0; 6],
            keypad_drawer_puzzle_state: [0; 3],
            lightbulb_unlock: None,
        }
    }
}

#[derive(Component)]
struct DebugSceneText;

fn scene_transition_system(
    mut player: ResMut<Player>,
    mut load_scene: EventWriter<LoadScene>,
    mut transition_to_scene: EventReader<TransitionToScene>,
) {
    let next_state = match &mut player.scene {
        SceneState::Transitioning(_, next, tick) => {
            *tick += 1;
            if *tick == TICKS_PER_TRANSITION {
                load_scene.send(LoadScene(*next));
                Some(SceneState::Active(*next))
            } else {
                None
            }
        }
        SceneState::Active(prev) => {
            let mut state = None;
            for item in transition_to_scene.read() {
                if item.0 != *prev {
                    state = Some(SceneState::Transitioning(*prev, item.0, 0));
                }
            }
            state
        }
    };
    if let Some(state) = next_state {
        player.scene = state;
    }
}

fn keyboard_input_system(
    keyboard: Keyboard,
    mut player: ResMut<Player>,
    mut unload_scene: EventWriter<UnloadScene>,
) {
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
                    unload_scene.send(UnloadScene(scene));
                    return;
                }
            }
        }
    }
}

fn render_bg_system(
    player: Res<Player>,
    mut bg0: Query<(&mut Handle<Image>, &mut Sprite), (With<Background0>, Without<Background1>)>,
    mut bg1: Query<(&mut Handle<Image>, &mut Sprite), (With<Background1>, Without<Background0>)>,
    asset_server: Res<AssetServer>,
) {
    let mut bg0 = bg0.get_single_mut().unwrap();
    let mut bg1 = bg1.get_single_mut().unwrap();
    match player.scene {
        SceneState::Active(id) => {
            *bg0.0 = asset_server.load(id.asset_path());
            bg0.1.color.set_alpha(1.0);
            bg1.1.color.set_alpha(0.0);
        }
        SceneState::Transitioning(prev, next, tick) => {
            let next_alpha = tick as f32 / TICKS_PER_TRANSITION as f32;
            let prev_alpha = 1.0 - next_alpha;
            *bg0.0 = asset_server.load(prev.asset_path());
            bg0.1.color.set_alpha(prev_alpha);
            *bg1.0 = asset_server.load(next.asset_path());
            bg1.1.color.set_alpha(next_alpha);
        }
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
                -HORIZONTAL_RESOLUTION / 2.0,
                VERTICAL_RESOLUTION / 2.0 * 9.0 / 10.0,
                RenderLayer::DebugText.z(),
            )),
            ..default()
        },
        DebugSceneText,
    ));
}

fn setup(
    mut commands: Commands,
    mut load_scene: EventWriter<LoadScene>,
    mut reset_universe: EventWriter<ResetUniverse>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.0,
                0.0,
                RenderLayer::Background.z(),
            )),
            ..default()
        },
        Background0,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.0,
                0.0,
                RenderLayer::Background.z(),
            )),
            ..default()
        },
        Background1,
    ));
    load_scene.send(LoadScene(SceneId::Desk));
    reset_universe.send(ResetUniverse);
}

fn unload_scene_items(
    mut commands: Commands,
    items: Query<(Entity, &SceneItem)>,
    mut unload_scene: EventReader<UnloadScene>,
) {
    for unload_scene in unload_scene.read() {
        for item in items.iter() {
            let UnloadScene(scene) = unload_scene;
            let SceneItem(item_scene) = item.1;
            if item_scene == scene {
                commands.entity(item.0).despawn();
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<LoadScene>();
    app.add_event::<UnloadScene>();
    app.add_event::<ResetUniverse>();
    app.add_event::<TransitionToScene>();
    app.insert_resource(Player::new());
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            scene_transition_system,
            keyboard_input_system,
            render_bg_system,
            unload_scene_items,
        )
            .chain()
            .in_set(UpdateSet::PreScene),
    );
    if cfg!(feature = "debug_state") {
        app.add_systems(Startup, debug_setup);
        app.add_systems(Update, (debug_update).in_set(UpdateSet::Debug));
    }
}
