use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, Player, ResetUniverse, SceneItem, SceneState},
};

const PUZZLE_BUTTON_X_POSITIONS: [f32; 6] = [-105.0, -66.0, -24.0, 20.0, 59.0, 100.0];

const PUZZLE_BUTTON_Y_POSITION: f32 = -209.0;
const PUZZLE_BUTTON_WIDTH: f32 = 45.0;
const PUZZLE_BUTTON_HEIGHT: f32 = 45.0;

const NUM_PUZZLE_CHARACTERS: usize = 12;
const PUZZLE_CHARACTERS: [&'static str; NUM_PUZZLE_CHARACTERS] =
    ["A", "B", "D", "E", "H", "I", "N", "O", "S", "R", "P", "U"];

#[derive(Component)]
struct PuzzleSegment {
    word_pos: usize,     // position within solution word
    sequence_pos: usize, // position within list of selectable characters
}

#[derive(Component)]
struct RestartUniverseButton;

fn load_scene(
    mut commands: Commands,
    mut load_scene: EventReader<LoadScene>,
    player: Res<Player>,
    asset_server: Res<AssetServer>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Behind {
            commands.spawn((
                ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                    Vec2::new(-537.0, -37.0),
                    300.0,
                    600.0,
                )),
                RestartUniverseButton,
                SceneItem(SceneId::Behind),
            ));

            let style = TextStyle {
                font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                font_size: 60.0,
                color: Color::linear_rgb(0.2, 0.9, 0.4),
                ..default()
            };

            for (index, x) in PUZZLE_BUTTON_X_POSITIONS.iter().enumerate() {
                let y = PUZZLE_BUTTON_Y_POSITION;

                let puzzle_segment = player.behind_puzzle_state[index];
                let puzzle_segment_char = PUZZLE_CHARACTERS[puzzle_segment];
                let rect = Rectangle::from_pos_width_height(
                    Vec2::new(*x, y),
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
                        transform: Transform::from_translation(Vec3::new(*x, y, 5.0)),
                        ..default()
                    },
                    ClickableShape::Rectangle(rect),
                    SceneItem(SceneId::Behind),
                ));
            }
        }
    }
}

fn update_puzzle(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut puzzle_segments: Query<(&mut PuzzleSegment, &mut Text, &ClickableShape)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for mut puzzle_segment in puzzle_segments.iter_mut() {
        if puzzle_segment.2.contains(mouse_pos.0) {
            let mut sequence = puzzle_segment.0.sequence_pos;
            sequence += 1;
            if sequence == NUM_PUZZLE_CHARACTERS {
                sequence = 0;
            }
            puzzle_segment.0.sequence_pos = sequence;

            puzzle_segment.1.sections[0].value = String::from(PUZZLE_CHARACTERS[sequence]);

            player.behind_puzzle_state[puzzle_segment.0.word_pos] = sequence;
        }
    }
}

fn update_universe_button(
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    button: Query<&ClickableShape, With<RestartUniverseButton>>,
    mut reset_universe: EventWriter<ResetUniverse>
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    if let Ok(button) = button.get_single() {
        if button.contains(mouse_pos.0) {
            // TODO: Wire up other reset logic here
            *player = Player::new();
            player.scene = SceneState::ForceTransition(SceneId::Behind, SceneId::Desk);
            reset_universe.send(ResetUniverse);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update_puzzle, update_universe_button));
}
