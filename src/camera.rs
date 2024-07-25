use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1080.0);
    commands.spawn((
        camera,
        MainCamera,
        crate::pixelate::PixelateSettings {
            block_size: 2.0,
            ..default()
        },
    ));
}

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
