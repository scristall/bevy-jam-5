use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct TvMonster;

#[derive(Component)]
pub struct TvPlayer;

fn setup(
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
        crate::mask::MaskSettings {
            ..default()
        },
    ));

    let material_handle = materials.add(ColorMaterial {
        color: Color::WHITE.into(),
        texture: Some(image_handle),
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(400.0, 300.0))),
        material: material_handle,
        transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
            -400.0, 0.0, 2.0,
        ),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/compass.png"),
            ..Default::default()
        },
        first_pass_layer.clone(),
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/tv_monster.png"),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                100.0, 0.0, 2.0,
            ),
            ..Default::default()
        },
        first_pass_layer.clone(),
        TvMonster,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/tv_player.png"),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -100.0, 0.0, 2.0,
            ),
            ..Default::default()
        },
        first_pass_layer.clone(),
        TvPlayer,
    ));
}

fn update(
    mut tv_player: Query<&mut Transform, With<TvPlayer>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut tv_player = tv_player.single_mut();

    if keyboard.pressed(KeyCode::KeyA) {
        tv_player.translation.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        tv_player.translation.x += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        tv_player.translation.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyW) {
        tv_player.translation.y += 1.0;
    }
}

pub fn tv_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
}
