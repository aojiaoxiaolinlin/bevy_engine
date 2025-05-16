use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::{
        palettes::css::{GREEN, RED},
        Color,
    },
    image::Image,
    math::Vec3,
    prelude::{Camera, Camera2d, Commands, Mesh, Mesh2d, Rectangle, ResMut, Transform},
    render::{
        mesh::MeshAabb,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{AlphaMode2d, ColorMaterial, MeshMaterial2d},
    DefaultPlugins,
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
    let mesh = Mesh::from(Rectangle::default());

    let transform = Transform::from_scale(Vec3::splat(128.0));
    let affine = transform.compute_affine();
    let aabb = mesh.compute_aabb().unwrap();
    let min = affine.transform_point3a(aabb.min());
    let max = affine.transform_point3a(aabb.max());

    let size = Extent3d {
        width: (max.x - min.x) as u32,
        height: (max.y - min.y) as u32,
        ..Default::default()
    };
    let mut texture = Image {
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
    texture.resize(size);
    let first_pass_layer = RenderLayers::layer(1);
    let texture_handle = images.add(texture);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        transform,
        first_pass_layer.clone(),
    ));
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            target: texture_handle.clone().into(),
            clear_color: Color::NONE.into(),
            ..Default::default()
        },
        first_pass_layer,
    ));

    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(texture_handle),
            color: Color::from(GREEN),
            alpha_mode: AlphaMode2d::Blend,
            ..Default::default()
        })),
        Transform::from_scale(Vec3::splat(128.0)),
    ));
}
