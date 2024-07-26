use bevy::{prelude::*, render::camera::ScalingMode};

const PIXELATE_BLOCK_SIZE: f32 = 2.0;

#[derive(Component)]
pub struct MainCamera;

#[allow(clippy::field_reassign_with_default)]
fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1080.0);
    let mut settings = crate::pixelate::PixelateSettings::default();
    settings.block_size = PIXELATE_BLOCK_SIZE;

    commands.spawn((camera, MainCamera, settings));
}

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
