use crate::camera::{HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};
use crate::components::{ClickableShape, Rectangle};
use crate::gamedata::{AmRadioFreq, SceneId};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Radio {
    pub scene: SceneId,
    pub area: ClickableShape,
    pub freq: AmRadioFreq,
}

impl Radio {
    pub fn new() -> Self {
        let area = Rectangle {
            top_left: Vec2::new(0.0, 0.0),
            bottom_right: Vec2::new(HORIZONTAL_RESOLUTION / 2.0, -VERTICAL_RESOLUTION / 2.0),
        };
        Self {
            scene: SceneId::Desk,
            area: area.into(),
            freq: AmRadioFreq(0),
        }
    }
}
