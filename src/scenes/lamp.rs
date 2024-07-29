use bevy::prelude::*;

use crate::{
    gamedata::SceneId,
    player::{LightbulbColor, LoadScene, Player, SceneItem},
};

fn load_scene(
    mut commands: Commands,
    mut player: ResMut<Player>,
    asset_server: Res<AssetServer>,
    mut load_scene: EventReader<LoadScene>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Lamp {
            player.installed_lightbulb = player.lightbulb_unlock;
            if let Some(LightbulbColor::Red) = player.installed_lightbulb {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("images/scenes/lamp_red.png"),
                        transform: Transform::from_xyz(0.0, 0.0, 3.0),
                        ..Default::default()
                    },
                    SceneItem(SceneId::Lamp),
                ));
            }
        }
    }
}

fn update() {}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
