use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::{RED, WHITE},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1,
        height: 1,
        ..Default::default()
    };
    let texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("滤镜纹理"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        },
        ..Default::default()
    };

    let render_layers = RenderLayers::layer(1);
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            target: images.add(texture).into(),
            ..Default::default()
        },
        render_layers.clone(),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(100., 100.)))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        render_layers.clone(),
    ));

    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform::from_scale(Vec3::splat(64.0)).with_translation(Vec3::new(40.0, 0.0, 0.0)),
    ));
}
