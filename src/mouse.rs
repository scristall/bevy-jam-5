use bevy::{prelude::*, window::PrimaryWindow};

use crate::camera::MainCamera;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct CursorText;

fn update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut text: Query<&mut Text, With<CursorText>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for mut text in &mut text {
            text.sections[1].value = format!("{:.0}/{:.0}", world_position.x, world_position.y);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "x/y: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
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
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
