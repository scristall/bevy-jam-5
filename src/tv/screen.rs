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
};

#[derive(Component)]
pub struct TvBackground;

#[derive(Component)]
pub struct TvScreenMaterial(pub Handle<ColorMaterial>);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
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
        //crate::mask::MaskSettings { ..default() },
    ));

    let material_handle = materials.add(ColorMaterial {
        color: Color::WHITE,
        texture: Some(image_handle),
    });

    commands.spawn(TvScreenMaterial(material_handle.clone()));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/tv/compass.png"),
            ..Default::default()
        },
        TvBackground,
        first_pass_layer.clone(),
    ));
}

pub fn screen_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
