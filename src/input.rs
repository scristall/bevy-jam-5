use bevy::{prelude::*, window::PrimaryWindow};
use bevy::sprite::Anchor;

use crate::camera::{MainCamera, HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};

#[cfg(feature = "debug_input")]
type Keyboard<'a> = Res<'a, ButtonInput<KeyCode>>;

// #[cfg(feature = "debug_input")]
// type DebugText<'world, 'state, 'text> = ParamSet<'world, 'state, (
//         Query<'world, 'state, &'text mut Text, With<DebugCursorPosText>>,
//         Query<'world, 'state, &'text mut Text, With<DebugKeyInputText>>,
// )>;

#[derive(Resource, Default)]
pub struct MousePosition(Vec2);

#[derive(Component)]
struct DebugKeyInputText;

#[derive(Component)]
struct DebugCursorPosText;

#[cfg(not(feature = "debug_input"))]
fn update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut coords: ResMut<MousePosition>,
) {
    always_update(q_window, q_camera, &mut coords)
}

#[cfg(feature = "debug_input")]
fn update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    keyboard: Keyboard,
    mut coords: ResMut<MousePosition>,
    mut debug_text: ParamSet<(
        Query<&mut Text, With<DebugCursorPosText>>,
        Query<&mut Text, With<DebugKeyInputText>>,
    )>,
) {
    always_update(q_window, q_camera, &mut coords);
    debug_update(coords.0, &keyboard, &mut debug_text)
}

fn always_update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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
    }
}

#[cfg(feature = "debug_input")]
fn debug_update(
    cursor: Vec2,
    keyboard: &Keyboard,
    debug_text: &mut ParamSet<(
        Query<&mut Text, With<DebugCursorPosText>>,
        Query<&mut Text, With<DebugKeyInputText>>,
    )>,
) {
    use std::fmt::Write;
    for mut text in debug_text.p0().iter_mut() {
        text.sections[1].value.clear();
        write!(
            &mut text.sections[1].value,
            "{:.0}/{:.0}",
            cursor.x, cursor.y
        )
        .unwrap();
    }
    for mut text in debug_text.p1().iter_mut() {
        text.sections[1].value.clear();
        for key in keyboard.get_pressed() {
            write!(&mut text.sections[1].value, "{:?} ", key).unwrap();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    if cfg!(feature = "debug_input") {
        debug_setup(&mut commands, &asset_server)
    }
}

fn debug_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("fonts/FiraMono-Regular.ttf"),
        font_size: 20.0,
        ..default()
    };
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("x/y: ", style.clone()),
            TextSection::from_style(style.clone()),
        ]),
        DebugCursorPosText,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("key input: ", style.clone()),
                TextSection::from_style(style.clone()),
            ]).with_justify(JustifyText::Left),
            text_anchor: Anchor::CenterLeft,
            transform: Transform::from_translation(Vec3::new(
                -HORIZONTAL_RESOLUTION / 2.0,
                VERTICAL_RESOLUTION / 2.0 * 9.5 / 10.0,
                0.0,
            )),
            ..default()
        },
        DebugKeyInputText,
    ));
}

pub fn plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    app.add_systems(Startup, setup);
    // The MousePosition and KeyInput resources will be extensively used by other systems; update them first!
    app.add_systems(PreUpdate, update);
}
