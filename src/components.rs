use bevy::prelude::*;
use enum_dispatch::enum_dispatch;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateSet {
    Input,
    Debug,
    PreScene,
    Scene,
    PostScene,
}

impl UpdateSet {
    pub const fn canon_order() -> (Self, Self, Self, Self, Self) {
        use UpdateSet::*;
        (Input, Debug, PreScene, Scene, PostScene)
    }
}

pub type Keyboard<'a> = Res<'a, ButtonInput<KeyCode>>;

#[enum_dispatch]
pub trait ClickableArea {
    fn contains(&self, pos: Vec2) -> bool;
}

#[enum_dispatch(ClickableArea)]
#[derive(Component)]
pub enum ClickableShape {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Component)]
pub struct Rectangle {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

impl ClickableArea for Rectangle {
    fn contains(&self, pos: Vec2) -> bool {
        pos.x >= self.top_left.x
            && pos.x <= self.bottom_right.x
            && pos.y <= self.top_left.y
            && pos.y >= self.bottom_right.y
    }
}

#[derive(Component)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl ClickableArea for Circle {
    fn contains(&self, pos: Vec2) -> bool {
        pos.distance_squared(self.center) <= self.radius * self.radius
    }
}
