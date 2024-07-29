use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LightbulbColor, LoadScene, Player, SceneItem},
};

const BUTTONS: [Vec2; 9] = [
    Vec2::new(-41.0, 30.0),
    Vec2::new(142.0, 35.0),
    Vec2::new(329.0, 28.0),
    Vec2::new(-74.0, -138.0),
    Vec2::new(134.0, -141.0),
    Vec2::new(333.0, -159.0),
    Vec2::new(-91.0, -345.0),
    Vec2::new(135.0, -345.0),
    Vec2::new(327.0, -356.0),
];

#[derive(Component)]
pub struct Button(usize);

fn load_scene(
    mut commands: Commands,
    player: Res<Player>,
    asset_server: Res<AssetServer>,
    mut load_scene: EventReader<LoadScene>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Phone {
            if let Some(LightbulbColor::Red) = player.installed_lightbulb {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("images/scenes/phone_red.png"),
                        transform: Transform::from_xyz(0.0, 0.0, 3.0),
                        ..Default::default()
                    },
                    SceneItem(SceneId::Phone),
                ));

                for (index, button) in BUTTONS.iter().enumerate() {
                    commands.spawn((
                        ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                            Vec2::new(button.x, button.y),
                            50.0,
                            50.0,
                        )),
                        Button(index),
                        SceneItem(SceneId::Phone),
                    ));
                }
            }
        }
    }
}

fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    buttons: Query<(&ClickableShape, &Button)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for button in buttons.iter() {
        if button.0.contains(mouse_pos.0) {
            player.dialed_numbers.push(button.1 .0);
        }

        if player.dialed_numbers.len() > 9 {
            player.dialed_numbers.remove(0);
        }

        let solution = vec![8, 5, 3, 1, 7, 6, 0, 2, 5];

        let matching = player
            .dialed_numbers
            .iter()
            .zip(&solution)
            .filter(|&(a, b)| a == b)
            .count();
        if matching == 9 {
            commands.spawn((SpriteBundle {
                texture: asset_server.load("images/scenes/universe_saved.png"),
                transform: Transform::from_xyz(0.0, 0.0, 20.0),
                ..Default::default()
            },));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
