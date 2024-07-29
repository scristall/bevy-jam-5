use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, SceneItem},
};

const PUZZLE_BUTTON_X_POSITIONS: [f32; 6] = [-340.0, -225.0, -87.0, 50.0, 174.0, 300.0];

const PUZZLE_BUTTON_Y_POSITION: f32 = -239.0;
const PUZZLE_BUTTON_WIDTH: f32 = 100.0;
const PUZZLE_BUTTON_HEIGHT: f32 = 100.0;

const NUM_PUZZLE_CHARACTERS: usize = 12;
const PUZZLE_CHARACTERS: [&'static str; NUM_PUZZLE_CHARACTERS] =
    ["A", "B", "D", "E", "H", "I", "N", "O", "S", "R", "P", "U"];

#[derive(Component)]
struct PuzzleSegment(usize);

fn load_scene(
    mut commands: Commands,
    mut load_scene: EventReader<LoadScene>,
    asset_server: Res<AssetServer>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Behind {
            let style = TextStyle {
                font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                font_size: 100.0,
                color: Color::BLACK,
                ..default()
            };

            for x in PUZZLE_BUTTON_X_POSITIONS {
                let y = PUZZLE_BUTTON_Y_POSITION;
                let left = x - PUZZLE_BUTTON_WIDTH / 2.0;
                let right = x + PUZZLE_BUTTON_WIDTH / 2.0;
                let top = y + PUZZLE_BUTTON_HEIGHT / 2.0;
                let bottom = y - PUZZLE_BUTTON_HEIGHT / 2.0;

                let puzzle_segment = PUZZLE_CHARACTERS[0];

                commands.spawn((
                    PuzzleSegment(0),
                    Text2dBundle {
                        text: Text::from_sections([TextSection::new(
                            puzzle_segment,
                            style.clone(),
                        )])
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_translation(Vec3::new(x, y, 5.0)),
                        ..default()
                    },
                    ClickableShape::Rectangle(Rectangle {
                        top_left: Vec2::new(left, top),
                        bottom_right: Vec2::new(right, bottom),
                    }),
                    SceneItem(SceneId::Behind),
                ));
            }
        }
    }
}

fn update(
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut puzzle_segments: Query<(&mut PuzzleSegment, &mut Text, &ClickableShape)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for mut puzzle_segment in puzzle_segments.iter_mut() {
        if let ClickableShape::Rectangle(rect) = puzzle_segment.2 {
            if rect.contains(mouse_pos.0) {
                let mut index = puzzle_segment.0 .0;
                index += 1;
                if index == NUM_PUZZLE_CHARACTERS {
                    index = 0;
                }
                *puzzle_segment.0 = PuzzleSegment(index);

                puzzle_segment.1.sections[0].value = String::from(PUZZLE_CHARACTERS[index]);
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
