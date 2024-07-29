use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, Player, SceneItem},
};

const PUZZLE_BUTTON_X_POSITIONS: [f32; 3] = [215.0, 294.0, 376.0];

const PUZZLE_BUTTON_Y_POSITION: f32 = 177.0;
const PUZZLE_BUTTON_WIDTH: f32 = 100.0;
const PUZZLE_BUTTON_HEIGHT: f32 = 100.0;

const NUM_PUZZLE_CHARACTERS: usize = 12;
const PUZZLE_CHARACTERS: [&'static str; NUM_PUZZLE_CHARACTERS] =
    ["A", "B", "D", "E", "H", "I", "N", "O", "S", "R", "P", "U"];

#[derive(Component)]
struct UpButton(usize);

#[derive(Component)]
struct DownButton(usize);

#[derive(Component)]
struct PuzzleSegment {
    word_pos: usize,     // position within solution word
    sequence_pos: usize, // position within list of selectable characters
}

fn load_scene(
    mut commands: Commands,
    mut load_scene: EventReader<LoadScene>,
    player: Res<Player>,
    asset_server: Res<AssetServer>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::KeypadDrawer {
            let style = TextStyle {
                font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                font_size: 100.0,
                color: Color::BLACK,
                ..default()
            };

            for (index, x) in PUZZLE_BUTTON_X_POSITIONS.iter().enumerate() {
                let y = PUZZLE_BUTTON_Y_POSITION;

                let puzzle_segment = player.keypad_drawer_puzzle_state[index];
                let puzzle_segment_char = PUZZLE_CHARACTERS[puzzle_segment];
                let up_rect = Rectangle::from_pos_width_height(
                    Vec2::new(*x + 45.0, y + 180.0 + (5 * index) as f32),
                    PUZZLE_BUTTON_WIDTH,
                    PUZZLE_BUTTON_HEIGHT,
                );
                let down_rect = Rectangle::from_pos_width_height(
                    Vec2::new(*x + 30.0, y - 224.0),
                    PUZZLE_BUTTON_WIDTH,
                    PUZZLE_BUTTON_HEIGHT,
                );

                commands.spawn((
                    PuzzleSegment {
                        word_pos: index,
                        sequence_pos: puzzle_segment,
                    },
                    Text2dBundle {
                        text: Text::from_sections([TextSection::new(
                            puzzle_segment_char,
                            style.clone(),
                        )])
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_translation(Vec3::new(
                            *x,
                            y + (5 * index) as f32,
                            5.0,
                        )),
                        ..default()
                    },
                    SceneItem(SceneId::KeypadDrawer),
                ));
                commands.spawn((
                    ClickableShape::Rectangle(up_rect),
                    UpButton(index),
                    SceneItem(SceneId::KeypadDrawer),
                ));
                commands.spawn((
                    ClickableShape::Rectangle(down_rect),
                    DownButton(index),
                    SceneItem(SceneId::KeypadDrawer),
                ));
            }
        }
    }
}

pub fn is_keypad_drawer_solved(player: &Player) -> bool {
    PUZZLE_CHARACTERS[player.keypad_drawer_puzzle_state[0]] == "S"
        && PUZZLE_CHARACTERS[player.keypad_drawer_puzzle_state[1]] == "E"
        && PUZZLE_CHARACTERS[player.keypad_drawer_puzzle_state[2]] == "E"
}

fn update(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    up_buttons: Query<(&ClickableShape, &UpButton)>,
    down_buttons: Query<(&ClickableShape, &DownButton)>,
    mut puzzle_segments: Query<(&mut PuzzleSegment, &mut Text)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    if is_keypad_drawer_solved(&player) {
      return;
    }

    for mut puzzle_segment in puzzle_segments.iter_mut() {
        for down_button in down_buttons.iter() {
            if down_button.0.contains(mouse_pos.0) && down_button.1 .0 == puzzle_segment.0.word_pos
            {
                let mut sequence = puzzle_segment.0.sequence_pos;
                if sequence == 0 {
                    sequence = NUM_PUZZLE_CHARACTERS - 1;
                } else {
                    sequence -= 1;
                }
                puzzle_segment.0.sequence_pos = sequence;

                puzzle_segment.1.sections[0].value = String::from(PUZZLE_CHARACTERS[sequence]);

                player.keypad_drawer_puzzle_state[puzzle_segment.0.word_pos] = sequence;
            }
        }
        for up_button in up_buttons.iter() {
            if up_button.0.contains(mouse_pos.0) && up_button.1 .0 == puzzle_segment.0.word_pos {
                let mut sequence = puzzle_segment.0.sequence_pos;
                sequence += 1;
                if sequence == NUM_PUZZLE_CHARACTERS {
                    sequence = 0;
                }
                puzzle_segment.0.sequence_pos = sequence;

                puzzle_segment.1.sections[0].value = String::from(PUZZLE_CHARACTERS[sequence]);

                player.keypad_drawer_puzzle_state[puzzle_segment.0.word_pos] = sequence;
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
