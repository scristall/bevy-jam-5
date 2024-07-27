use bevy::{prelude::*, render::view::RenderLayers};
use bevy_kira_audio::prelude::*;

use super::{screen::TvBackground, tv_monster::TvMonster, whirlpool::Whirlpool};

#[derive(Component)]
pub struct TvPlayer;

#[derive(Component)]
pub struct TvControlled {
    pub puzzle_pos: u32,
}

#[derive(Component)]
pub struct TvFalling;

enum Direction {
    North,
    East,
    South,
    West,
}

const EDGE_GAP: f32 = 10.0;
const X_BOUND: f32 = 190.0;
const Y_BOUND: f32 = 140.0;

const PUZZLE_SOLVED: u32 = 3;

fn check_puzzle(puzzle_pos: u32, dir: Direction) -> u32 {
    if puzzle_pos == PUZZLE_SOLVED {
        return PUZZLE_SOLVED;
    }

    match dir {
        Direction::North => 0,
        Direction::East => {
            if puzzle_pos == 0 {
                1
            } else {
                0
            }
        }
        Direction::South => {
            if puzzle_pos == 1 {
                2
            } else {
                0
            }
        }
        Direction::West => {
            if puzzle_pos == 2 {
                3
            } else {
                0
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/tv_player.png"),
            transform: Transform::from_xyz(-160.0, 0.0, 2.0),
            ..Default::default()
        },
        TvPlayer,
        RenderLayers::layer(1),
    ));
}

fn update(
    mut commands: Commands,
    mut controlled: Query<(&mut Transform, &mut TvControlled, Entity, &Handle<Image>)>,
    tv_backgrounds: Query<(Entity, &TvBackground)>,
    uncontrolled: Query<Entity, (Or<(With<TvPlayer>, With<TvMonster>)>, Without<TvControlled>)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut controlled) = controlled.get_single_mut() {
        if keyboard.pressed(KeyCode::KeyA) {
            controlled.0.translation.x -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            controlled.0.translation.x += 1.0;
        }

        if keyboard.pressed(KeyCode::KeyS) {
            controlled.0.translation.y -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyW) {
            controlled.0.translation.y += 1.0;
        }

        let mut screen_change_dir = None;

        if controlled.0.translation.x > X_BOUND {
            controlled.0.translation.x = -X_BOUND + EDGE_GAP;
            screen_change_dir = Some(Direction::East);
        }
        if controlled.0.translation.x < -X_BOUND {
            controlled.0.translation.x = X_BOUND - EDGE_GAP;
            screen_change_dir = Some(Direction::West);
        }
        if controlled.0.translation.y > Y_BOUND {
            controlled.0.translation.y = -Y_BOUND + EDGE_GAP;
            screen_change_dir = Some(Direction::North);
        }
        if controlled.0.translation.y < -Y_BOUND {
            controlled.0.translation.y = Y_BOUND - EDGE_GAP;
            screen_change_dir = Some(Direction::South);
        }

        if let Some(dir) = screen_change_dir {
            let next_pos = check_puzzle(controlled.1.puzzle_pos, dir);
            if next_pos != controlled.1.puzzle_pos {
                controlled.1.puzzle_pos = next_pos;
                if next_pos == PUZZLE_SOLVED {
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("images/whirl1.png"),
                            ..Default::default()
                        },
                        Whirlpool { speed: 0.1 },
                        RenderLayers::layer(1),
                    ));
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("images/whirl2.png"),
                            ..Default::default()
                        },
                        Whirlpool { speed: 0.05 },
                        RenderLayers::layer(1),
                    ));
                    for tv_background in tv_backgrounds.iter() {
                        commands.entity(tv_background.0).despawn();
                    }
                    for uncontrolled in uncontrolled.iter() {
                        commands.entity(uncontrolled).despawn();
                    }
                }
            }
        }

        if controlled.1.puzzle_pos == PUZZLE_SOLVED
            && controlled.0.translation.distance(Vec3::new(0.0, 0.0, 0.0)) < 30.0
        {
            commands
                .entity(controlled.2)
                .remove::<TvControlled>()
                .insert(TvFalling);
            controlled.0.translation = Vec3::new(0.0, 0.0, 2.0);
            audio
                .play(asset_server.load("audio/tv_die_whirlpool.ogg"))
                .with_volume(0.3);
        }
    }
}

fn update_tv_falling(mut falling_players: Query<&mut Transform, With<TvFalling>>) {
    for mut falling_player in falling_players.iter_mut() {
        falling_player.scale *= 0.99;
    }
}

fn update_tv_player_falling(
    mut commands: Commands,
    falling_players: Query<&Transform, (With<TvFalling>, With<TvPlayer>)>,
    asset_server: Res<AssetServer>,
) {
    for falling_player in falling_players.iter() {
        if falling_player.scale.length() < 0.1 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::linear_rgb(1.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    texture: asset_server.load("images/tv_win.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 3.0),
                    ..Default::default()
                },
                RenderLayers::layer(1),
            ));
        }
    }
}

fn update_tv_monster_falling(
    mut commands: Commands,
    falling_players: Query<&Transform, (With<TvFalling>, With<TvMonster>)>,
    asset_server: Res<AssetServer>,
) {
    for falling_player in falling_players.iter() {
        if falling_player.scale.length() < 0.1 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        ..Default::default()
                    },
                    texture: asset_server.load("images/tv_win.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 3.0),
                    ..Default::default()
                },
                RenderLayers::layer(1),
            ));
        }
    }
}

pub fn tv_player_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            update,
            update_tv_falling,
            update_tv_player_falling,
            update_tv_monster_falling,
        ),
    );
}
