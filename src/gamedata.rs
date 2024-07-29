use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum RenderLayer {
    Background = 0,
    DebugText = 1,
}

pub fn debug_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Regular.ttf"),
        font_size: 20.0,
        color: Color::linear_rgb(1.0, 0.0, 0.0),
        ..default()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScenePlayerControl {
    TransitionSceneLeft,
    TransitionSceneRight,
    TransitionSceneBehind,
}

impl ScenePlayerControl {
    pub const fn key_code(self) -> KeyCode {
        use ScenePlayerControl::*;
        match self {
            TransitionSceneLeft => KeyCode::KeyA,
            TransitionSceneRight => KeyCode::KeyD,
            TransitionSceneBehind => KeyCode::KeyS,
        }
    }
}

macro_rules! scene_path {
    ($p:literal) => {
        concat!("images/scenes/", $p)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum SceneId {
    Desk,
    Radio,
    Tv,
    Lamp,
    KeypadDrawer,
    LockDrawer,
    BulletinBoard,
    Door,
    Behind,
}

impl SceneId {
    pub fn next_scene(self, control: ScenePlayerControl) -> Option<SceneId> {
        MAIN_RING_SCENES
            .iter()
            .copied()
            .position(|v| self == v)
            .map(|i| Self::main_ring_next_scene(i, control))
            .or_else(|| Self::try_escape_sub_scene(self, control))
    }

    const fn main_ring_next_scene(index: usize, control: ScenePlayerControl) -> SceneId {
        use ScenePlayerControl::*;
        let delta = match control {
            TransitionSceneLeft => -1,
            TransitionSceneRight => 1,
            TransitionSceneBehind => 2,
        };
        let index = index.overflowing_add_signed(delta).0 % MAIN_RING_SCENES.len();
        MAIN_RING_SCENES[index]
    }

    const fn try_escape_sub_scene(self, control: ScenePlayerControl) -> Option<SceneId> {
        use SceneId::*;
        match control {
            ScenePlayerControl::TransitionSceneBehind => (),
            _ => return None,
        }
        let scene = match self {
            Radio | Tv | Lamp | KeypadDrawer | LockDrawer => Desk,
            _ => return None,
        };
        Some(scene)
    }

    pub const fn asset_path(self) -> &'static str {
        use SceneId::*;
        match self {
            Desk => scene_path!("desk.png"),
            Radio => scene_path!("radio.png"),
            Tv => scene_path!("tv.png"),
            Lamp => scene_path!("lamp.png"),
            KeypadDrawer => scene_path!("locked_drawer_1.png"),
            LockDrawer => scene_path!("locked_drawer_2.png"),
            BulletinBoard => scene_path!("bulletin_board.png"),
            Door => scene_path!("door.png"),
            Behind => scene_path!("restart_universe_button.png"),
        }
    }
}

// Scenes are arranged left-to-right circularly.
const MAIN_RING_SCENES: [SceneId; 4] = [
    SceneId::Desk,
    SceneId::Door,
    SceneId::Behind,
    SceneId::BulletinBoard,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct AmRadioFreq(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetAmRadioFreq {
    Morse,
    Music,
    News,
    Numbers,
}

impl PresetAmRadioFreq {
    const fn value(self) -> i32 {
        match self {
            Self::Morse => 650,
            Self::Music => 750,
            Self::News => 610,
            Self::Numbers => 700,
        }
    }
}

impl From<PresetAmRadioFreq> for AmRadioFreq {
    fn from(v: PresetAmRadioFreq) -> Self {
        Self(v.value())
    }
}
