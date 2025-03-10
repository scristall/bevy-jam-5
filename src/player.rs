use crate::camera::{HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};
use crate::components::{ClickableArea, ClickableScene, ClickableShape, Keyboard, UpdateSet};
use crate::gamedata::{debug_text_style, RenderLayer, SceneId, ScenePlayerControl};
use crate::input::MousePosition;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fmt::Write;

const TICKS_PER_TRANSITION: u8 = 15;

#[derive(Debug, Clone)]
pub enum SceneState {
    Active(SceneId),
    Transitioning(SceneId, SceneId, u8),
    ForceTransition(SceneId, SceneId),
}

#[derive(Event)]
pub struct LoadScene(pub SceneId);

#[derive(Event)]
pub struct UnloadScene(pub SceneId);

#[derive(Event)]
pub struct ResetUniverse;

#[derive(Component)]
pub struct SceneItem(pub SceneId);

#[derive(Component)]
pub struct Background0;

#[derive(Component)]
pub struct Background1;

#[derive(Copy, Clone)]
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
    pub installed_lightbulb: Option<LightbulbColor>,
    pub right_speaker_broken: bool,
    pub has_surge_protector: bool,
    pub has_installed_surge_protector: bool,
    pub has_morse_code_translator: bool,
    pub has_installed_morse_code_translator: bool,
    pub has_key: bool,
    pub opened_key_drawer: bool,
    pub dialed_numbers: Vec<usize>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            scene: SceneState::Active(SceneId::Desk),
            behind_puzzle_state: [0; 6],
            keypad_drawer_puzzle_state: [0; 3],
            lightbulb_unlock: None,
            installed_lightbulb: None,
            right_speaker_broken: false,
            has_surge_protector: false,
            has_installed_surge_protector: false,
            has_morse_code_translator: false,
            has_installed_morse_code_translator: false,
            has_key: false,
            opened_key_drawer: false,
            dialed_numbers: vec![],
        }
    }
}

#[derive(Component)]
struct DebugSceneText;

fn scene_transition_system(
    mut player: ResMut<Player>,
    mut load_scene: EventWriter<LoadScene>,
    mut unload_scene: EventWriter<UnloadScene>,
) {
    match &mut player.scene {
        SceneState::ForceTransition(prev, next) => {
            let prev = *prev;
            player.scene = SceneState::Transitioning(prev, *next, 0);
            unload_scene.send(UnloadScene(prev));
        }
        SceneState::Transitioning(prev, next, tick) => {
            *tick += 1;
            if *tick == TICKS_PER_TRANSITION {
                load_scene.send(LoadScene(*next));
                player.scene = SceneState::Active(*next);
            }
        }
        _ => (),
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

fn check_clickable_scenes(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    clickables: Query<(&ClickableShape, &ClickableScene)>,
    mut unload_scene: EventWriter<UnloadScene>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for clickables in clickables.iter() {
        if clickables.0.contains(mouse_pos.0) {
            player.scene = SceneState::Transitioning(clickables.1.from, clickables.1.to, 0);
            unload_scene.send(UnloadScene(clickables.1.from));
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
        SceneState::ForceTransition(id, _) | SceneState::Active(id) => {
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
    app.insert_resource(Player::new());
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            keyboard_input_system,
            scene_transition_system,
            render_bg_system,
            unload_scene_items,
            check_clickable_scenes,
        )
            .chain()
            .in_set(UpdateSet::PreScene),
    );
    if cfg!(feature = "debug_state") {
        app.add_systems(Startup, debug_setup);
        app.add_systems(Update, (debug_update).in_set(UpdateSet::Debug));
    }
}
