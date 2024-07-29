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
    player::{LoadScene, Player, SceneItem},
    right_speaker::SmokeSpawner,
    tv::TvScreenMaterial,
};

use super::keypad_drawer::is_keypad_drawer_solved;

fn skewed_rectangle_builder(rect: Rectangle) -> Mesh {
    let [hw, hh] = [rect.half_size.x, rect.half_size.y];
    let positions = vec![
        [hw, hh, 0.0],
        [-hw, hh * 0.82, 0.0],
        [-hw, -hh * 0.82, 0.0],
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

struct DeskClickable {
    scene: SceneId,
    text: &'static str,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl DeskClickable {
    pub const fn new(
        scene: SceneId,
        text: &'static str,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> Self {
        Self {
            scene,
            text,
            left,
            top,
            right,
            bottom,
        }
    }
}

const DESK_CLICKABLES: [DeskClickable; 6] = [
    DeskClickable::new(SceneId::Tv, "TV", 335.0, 240.0, 925.0, -200.0),
    DeskClickable::new(SceneId::Lamp, "Lamp", -710.0, 415.0, -200.0, 155.0),
    DeskClickable::new(SceneId::Radio, "Radio", -290.0, 150.0, 280.0, -210.0),
    DeskClickable::new(SceneId::Phone, "Phone", -850.0, 80.0, -300.0, -200.0),
    DeskClickable::new(
        SceneId::LockDrawer,
        "Top Drawer",
        -831.0,
        -243.0,
        -498.0,
        -380.0,
    ),
    DeskClickable::new(
        SceneId::KeypadDrawer,
        "Bottom Drawer",
        -828.0,
        -384.0,
        -501.0,
        -516.0,
    ),
];

fn load_scene(
    mut commands: Commands,
    mut load_scene: EventReader<LoadScene>,
    mut meshes: ResMut<Assets<Mesh>>,
    tv_screen: Query<&TvScreenMaterial>,
    asset_server: Res<AssetServer>,
    player: Res<Player>,
) {
    for load_scene in load_scene.read() {
        if load_scene.0 != SceneId::Desk {
            return;
        }
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/scenes/desk_top_layer.png"),
                transform: Transform::from_xyz(0.0, 0.0, 4.0),
                ..Default::default()
            },
            SceneItem(SceneId::Desk),
        ));
        if let Ok(TvScreenMaterial(tv_screen)) = tv_screen.get_single() {
            let mesh = skewed_rectangle_builder(Rectangle::new(250.0, 242.0));

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(mesh)),
                    material: tv_screen.clone(),
                    transform: Transform::from_xyz(420.0, 0.0, 2.0),
                    ..default()
                },
                SceneItem(SceneId::Desk),
            ));
        }

        if player.right_speaker_broken {
            commands.spawn((SmokeSpawner::new(), SceneItem(SceneId::Desk)));
        }

        for dc in DESK_CLICKABLES.iter() {
            let to_scene = match dc.scene {
                SceneId::LockDrawer => {
                    if player.opened_key_drawer {
                        if player.has_morse_code_translator {
                            SceneId::LockDrawerEmpty
                        } else {
                            SceneId::LockDrawerSolved
                        }
                    } else {
                        SceneId::LockDrawer
                    }
                }
                SceneId::KeypadDrawer => {
                    if is_keypad_drawer_solved(&player) {
                        if player.has_surge_protector {
                            SceneId::KeypadDrawerEmpty
                        } else {
                            SceneId::KeypadDrawerSolved
                        }
                    } else {
                        SceneId::KeypadDrawer
                    }
                }
                _ => dc.scene,
            };
            commands.spawn((
                SceneItem(SceneId::Desk),
                ClickableShape::from(components::Rectangle {
                    top_left: Vec2::new(dc.left, dc.top),
                    bottom_right: Vec2::new(dc.right, dc.bottom),
                }),
                ClickableLabel(dc.text),
                ClickableScene {
                    from: SceneId::Desk,
                    to: to_scene,
                },
            ));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, load_scene);
}
