use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::{
        palettes::css::{BLUE, GREEN, RED},
        Color, ColorToComponents,
    },
    math::Vec3,
    prelude::{Camera, Camera2d, Commands, Image, Mesh, Mesh2d, ResMut, Transform},
    render::{
        mesh::{Indices, MeshAabb, PrimitiveTopology},
        primitives::Aabb,
        render_asset::RenderAssetUsages,
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
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let position = vec![[0.0, 100.0, 0.0], [-100.0, 0.0, 0.0], [100.0, 0.0, 0.0]];
    let colors = vec![
        RED.to_f32_array(),
        GREEN.to_f32_array(),
        BLUE.to_f32_array(),
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    let aabb = mesh.compute_aabb().unwrap();
    let max = aabb.max();
    let min = aabb.min();
    let first_pass_layer = RenderLayers::layer(1);
    let transform = Transform::from_scale(Vec3::splat(4.0));
    let compute_affine = transform.compute_affine();
    let max = compute_affine.transform_point3a(max);
    let min = compute_affine.transform_point3a(min);
    let aabb = Aabb::from_min_max(min.into(), max.into());
    let width = (max.x - min.x).abs() as u32;
    let height = (max.y - min.y).abs() as u32;
    dbg!(width, height);
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        transform,
        first_pass_layer.clone(),
    ));

    let size = Extent3d {
        width,
        height,
        ..Default::default()
    };
    let mut texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    texture.resize(size);
    let image_handle = images.add(texture);
    commands.spawn((
        Camera2d,
        Camera {
            order: -1, // render before the main camera
            target: image_handle.clone().into(),
            clear_color: Color::NONE.into(),
            ..Default::default()
        },
        Transform::from_translation(aabb.center.into()),
        first_pass_layer,
    ));
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let position = vec![
        [100.0, 100.0, 0.0],
        [-100.0, 100.0, 0.0],
        [-100.0, 0.0, 0.0],
        [100.0, 0.0, 0.0],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position);
    mesh.insert_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
    );
    commands.spawn((
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(image_handle),
            color: Color::WHITE,
            alpha_mode: AlphaMode2d::Blend,
            ..Default::default()
        })),
        Mesh2d(meshes.add(mesh)),
        Transform::from_scale(Vec3::splat(4.0)),
    ));
    commands.spawn(Camera2d);
}
