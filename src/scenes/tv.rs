use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    components::{self, ClickableLabel, ClickableScene, ClickableShape},
    gamedata::SceneId,
    player::{LoadScene, SceneItem},
    tv::TvScreenMaterial,
};

fn skewed_rectangle_builder(rect: Rectangle) -> Mesh {
    let [hw, hh] = [rect.half_size.x, rect.half_size.y];
    let positions = vec![
        [hw, hh, 0.0],
        [-hw, hh * 0.84, 0.0],
        [-hw, -hh * 0.95, 0.0],
        [hw, -hh, 0.0],
    ];
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = vec![[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(indices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn load_scene(
    mut commands: Commands,
    mut load_scene: EventReader<LoadScene>,
    mut meshes: ResMut<Assets<Mesh>>,
    tv_screen: Query<&TvScreenMaterial>,
    asset_server: Res<AssetServer>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 == SceneId::Tv {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/scenes/tv_top_layer.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 4.0),
                    ..Default::default()
                },
                SceneItem(SceneId::Tv),
            ));
            if let Ok(TvScreenMaterial(tv_screen)) = tv_screen.get_single() {
                let mesh = skewed_rectangle_builder(Rectangle::new(1160.0, 1000.0));

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(mesh)),
                        material: tv_screen.clone(),
                        transform: Transform::from_xyz(194.0, 48.0, 2.0),
                        ..default()
                    },
                    SceneItem(SceneId::Tv),
                    ClickableShape::from(components::Rectangle {
                        top_left: Vec2::new(335.0, 240.0),
                        bottom_right: Vec2::new(925.0, -200.0),
                    }),
                ));
            }
        }
    }
}

fn update() {}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (load_scene, update));
}
