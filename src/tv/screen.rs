use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct TvBackground;

fn skewed_rectangle_builder(rect: Rectangle) -> Mesh {
    let [hw, hh] = [rect.half_size.x, rect.half_size.y];
    let positions = vec![
        [hw, hh * 0.75, 0.0],
        [-hw, hh, 0.0],
        [-hw, -hh, 0.0],
        [hw, -hh * 0.75, 0.0],
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Extent3d {
        width: 400,
        height: 300,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    let first_pass_layer = RenderLayers::layer(1);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(300.0);
    camera.camera.order = -1;
    camera.camera.target = image_handle.clone().into();
    camera.camera.clear_color = Color::WHITE.into();
    commands.spawn((
        camera,
        first_pass_layer.clone(),
        crate::mask::MaskSettings { ..default() },
    ));

    let material_handle = materials.add(ColorMaterial {
        color: Color::WHITE,
        texture: Some(image_handle),
    });

    let rect = Rectangle::new(400.0, 300.0);

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(rect)),
        material: material_handle.clone(),
        transform: Transform::from_xyz(
            -400.0, -200.0, 2.0,
        ),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(skewed_rectangle_builder(rect))),
        material: material_handle.clone(),
        transform: Transform::from_xyz(
            -400.0, 200.0, 2.0,
        ),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/compass.png"),
            ..Default::default()
        },
        TvBackground,
        first_pass_layer.clone(),
    ));
}

pub fn screen_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
