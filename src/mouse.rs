use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;

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
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;

        for mut text in &mut text {
            text.sections[1].value = format!("{:.0}/{:.0}", world_position.x, world_position.y);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
