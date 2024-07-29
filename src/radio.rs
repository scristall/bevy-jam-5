use crate::components::ClickableShape;
use crate::gamedata::AmRadioFreq;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Radio {
    pub area: ClickableShape,
    pub freq: AmRadioFreq,
}

impl Radio {
    pub fn new(area: ClickableShape) -> Self {
        Self {
            area,
            freq: AmRadioFreq(0),
        }
    }
}
