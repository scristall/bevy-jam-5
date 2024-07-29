use bevy::render::RenderSet::Render;
use bevy::sprite::Anchor;
use bevy::{prelude::*, window::PrimaryWindow};
use std::fmt::Write;

use crate::camera::{MainCamera, HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION};
use crate::components::{ClickableArea, ClickableLabel, ClickableShape, Keyboard, UpdateSet};
use crate::gamedata::{debug_text_style, highlight_text_style, RenderLayer};
use crate::player::Player;

// type DebugText<'world, 'state, 'text> = ParamSet<'world, 'state, (
//         Query<'world, 'state, &'text mut Text, With<DebugCursorPosText>>,
//         Query<'world, 'state, &'text mut Text, With<DebugKeyInputText>>,
// )>;

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

#[derive(Component)]
struct DebugKeyInputText;

#[derive(Component)]
struct DebugCursorPosText;

#[derive(Component)]
struct CursorText;

fn input_update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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
    }
}

fn highlight_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("", highlight_text_style(&asset_server))
                .with_justify(JustifyText::Left),
            text_anchor: Anchor::CenterLeft,
            ..default()
        },
        CursorText,
    ));
}

fn highlight_update(
    cursor: Res<MousePosition>,
    clickables: Query<(&ClickableShape, &ClickableLabel)>,
    mut text: Query<(&mut Text, &mut Transform), With<CursorText>>,
) {
    for mut text in &mut text {
        text.0.sections[0].value.clear();
    }
    for clickable in &clickables {
        if clickable.0.contains(cursor.0) {
            for mut text in &mut text {
                *text.1 = Transform::from_translation(Vec3::new(
                    cursor.0.x + 1.0,
                    cursor.0.y + 1.0,
                    RenderLayer::HighlightText.z(),
                ));
                write!(&mut text.0.sections[0].value, "{}", clickable.1 .0).unwrap();
            }
            return;
        }
    }
}

fn debug_update(
    cursor: Res<MousePosition>,
    keyboard: Keyboard,
    mut debug_text: ParamSet<(
        Query<&mut Text, With<DebugCursorPosText>>,
        Query<&mut Text, With<DebugKeyInputText>>,
    )>,
) {
    for mut text in debug_text.p0().iter_mut() {
        text.sections[1].value.clear();
        write!(
            &mut text.sections[1].value,
            "{:.0}/{:.0}",
            cursor.0.x, cursor.0.y
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

fn debug_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = debug_text_style(&asset_server);
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
            ])
            .with_justify(JustifyText::Left),
            text_anchor: Anchor::CenterLeft,
            transform: Transform::from_translation(Vec3::new(
                -HORIZONTAL_RESOLUTION / 2.0,
                VERTICAL_RESOLUTION / 2.0 * 9.5 / 10.0,
                RenderLayer::DebugText.z(),
            )),
            ..default()
        },
        DebugKeyInputText,
    ));
}

pub fn plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    // The MousePosition and KeyInput resources will be extensively used by other systems; update them first!
    app.add_systems(Startup, highlight_setup);
    app.add_systems(Update, (input_update).in_set(UpdateSet::Input));
    app.add_systems(Update, (highlight_update).in_set(UpdateSet::PostScene));
    if cfg!(feature = "debug_input") {
        app.add_systems(Startup, debug_setup);
        app.add_systems(Update, (debug_update).in_set(UpdateSet::Debug));
    }
}
