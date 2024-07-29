use bevy::prelude::*;

use crate::{
    components::{ClickableArea, ClickableLabel, ClickableShape, Rectangle},
    gamedata::SceneId,
    input::MousePosition,
    player::{LoadScene, Player, SceneItem},
};

#[derive(Component)]
struct Key;

#[derive(Component)]
struct Outlet;

fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Res<Player>,
    mut load_scene: EventReader<LoadScene>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::BulletinBoard {
            if !player.has_key {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("images/scenes/key.png"),
                        ..Default::default()
                    },
                    ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                        Vec2::new(-585.0, 347.0),
                        50.0,
                        300.0,
                    )),
                    ClickableLabel("Key"),
                    Key,
                    SceneItem(SceneId::BulletinBoard),
                ));
            }

            if player.has_surge_protector && !player.has_installed_surge_protector {
                commands.spawn((
                    ClickableShape::Rectangle(Rectangle::from_pos_width_height(
                        Vec2::new(255.0, -287.0),
                        100.0,
                        300.0,
                    )),
                    ClickableLabel("Install Protector"),
                    Outlet,
                    SceneItem(SceneId::BulletinBoard),
                ));
            }

            if player.has_installed_surge_protector {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("images/scenes/protected_outlet.png"),
                        ..Default::default()
                    },
                    SceneItem(SceneId::BulletinBoard),
                ));
            }
        }
    }
}

fn update_key_clickable(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    key: Query<(Entity, &ClickableShape), With<Key>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for key in key.iter() {
        if key.1.contains(mouse_pos.0) {
            player.has_key = true;
            commands.entity(key.0).despawn();
        }
    }
}

fn update_outlet_clickable(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mouse_pos: Res<MousePosition>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    key: Query<(Entity, &ClickableShape), With<Outlet>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for key in key.iter() {
        if key.1.contains(mouse_pos.0) {
            player.has_installed_surge_protector = true;
            commands.entity(key.0).despawn();
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/scenes/protected_outlet.png"),
                    ..Default::default()
                },
                SceneItem(SceneId::BulletinBoard),
            ));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (load_scene, update_key_clickable, update_outlet_clickable),
    );
}
