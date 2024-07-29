use crate::components::ClickableArea;
use crate::components::{ClickableAction, ClickableLabel, ClickableShape, UpdateSet};
use crate::gamedata::{highlight_text_style, RenderLayer};
use crate::input::MousePosition;
use crate::player::TransitionToScene;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fmt::Write;

mod behind;
mod bulletin_board;
mod desk;
mod door;
mod keypad_drawer;
mod lamp;
mod lock_drawer;
mod radio;
mod tv;

#[derive(Component)]
struct CursorText;

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

fn cursor_system(
    cursor: Res<MousePosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    clickables: Query<(&ClickableShape, &ClickableLabel, &ClickableAction)>,
    mut text: Query<(&mut Text, &mut Transform), With<CursorText>>,
    mut transitions: EventWriter<TransitionToScene>,
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
            if buttons.just_pressed(MouseButton::Left) {
                match clickable.2 {
                    ClickableAction::TransitionToScene(id) => {
                        transitions.send(TransitionToScene(*id));
                    }
                }
            }
            return;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        behind::plugin,
        bulletin_board::plugin,
        keypad_drawer::plugin,
        lamp::plugin,
        lock_drawer::plugin,
        tv::plugin,
        desk::plugin,
        radio::plugin,
    ));
    app.add_systems(Startup, highlight_setup);
    app.add_systems(Update, (cursor_system).in_set(UpdateSet::Scene));
}
