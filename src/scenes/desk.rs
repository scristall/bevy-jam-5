use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    gamedata::SceneId,
    player::{LoadScene, SceneItem},
    tv::TvScreenMaterial,
};

fn load_scene(
    mut commands: Commands,
    mut load_scene: EventReader<LoadScene>,
    mut meshes: ResMut<Assets<Mesh>>,
    tv_screen: Query<&TvScreenMaterial>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Desk {
            if let Ok(TvScreenMaterial(tv_screen)) = tv_screen.get_single() {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(400.0, 300.0))),
                        material: tv_screen.clone(),
                        transform: Transform::from_xyz(550.0, 0.0, 2.0),
                        ..default()
                    },
                    SceneItem(SceneId::Desk),
                ));
            }
        }
    }
}

fn update() {}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
