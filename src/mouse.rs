use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;
use std::fmt::Write;

#[derive(Component)]
struct CursorText;

#[derive(Resource, Default)]
pub struct MousePosition(Vec2);

fn update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut text: Query<&mut Text, With<CursorText>>,
    mut coords: ResMut<MousePosition>,
) {
    if cfg!(feature = "cursor_debug") {
        debug_update(&q_window, &q_camera, &mut text, &mut coords);
    }
}

fn debug_update(
    q_window: &Query<&Window, With<PrimaryWindow>>,
    q_camera: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    text: &mut Query<&mut Text, With<CursorText>>,
    coords: &mut ResMut<MousePosition>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;

        for mut text in text {
            text.sections[1].value.clear();
            write!(
                &mut text.sections[1].value,
                "{:.0}/{:.0}",
                world_position.x, world_position.y
            )
            .unwrap();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    if cfg!(feature = "cursor_debug") {
        debug_setup(&mut commands, &asset_server)
    }
}

fn debug_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "x/y: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                font_size: 20.0,
                ..default()
            }),
        ]),
        CursorText,
    ));
}

pub fn mouse_plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
