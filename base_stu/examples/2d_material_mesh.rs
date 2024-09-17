use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::{
        palettes::{
            css::{GREEN, PURPLE, RED},
            tailwind::BLUE_500,
        },
        Color, ColorToComponents,
    },
    math::Vec3,
    prelude::{default, Camera2dBundle, Commands, Image, Mesh, Rectangle, ResMut, Transform},
    render::render_asset::RenderAssetUsages,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
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
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh::from(Rectangle::default());

    let width = 256;
    let height = 256;
    let mut texture_data = vec![0; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let i: usize = (y * width + x) * 4;
            let t = y as f32 / (height - 1) as f32;
            texture_data[i] = (255.0 * t) as u8; // Red channel
            texture_data[i + 1] = (255.0 * (1.0 - t)) as u8; // Green channel
            texture_data[i + 2] = (255.0 * (1.0 - t)) as u8; // Blue channel
            texture_data[i + 3] = 255; // Alpha channel
        }
    }
    // 生成纹理
    let texture = Image::new(
        wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        wgpu::TextureDimension::D2,
        texture_data,
        wgpu::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );

    let texture = images.add(texture);
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::from_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial {
            texture: Some(texture),
            ..Default::default()
        }),
        ..Default::default()
    });
    let mut rect = Mesh::from(Rectangle::default());
    let colors = vec![
        Color::srgba_u8(216, 115, 15, 255)
            .to_linear()
            .to_f32_array(),
        Color::srgba_u8(216, 115, 15, 255)
            .to_linear()
            .to_f32_array(),
        Color::srgba_u8(216, 115, 15, 255)
            .to_linear()
            .to_f32_array(),
        Color::srgba_u8(216, 115, 15, 255)
            .to_linear()
            .to_f32_array(),
    ];
    rect.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(rect).into(),
        material: materials.add(ColorMaterial::default()),
        transform: Transform::from_scale(Vec3::splat(256.))
            .with_translation(Vec3::new(256., 0., 0.)),
        ..default()
    });

    let mut rect = Mesh::from(Rectangle::default());
    rect.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            RED.to_f32_array(),
            GREEN.to_f32_array(),
            BLUE_500.to_f32_array(),
            PURPLE.to_f32_array(),
        ],
    );
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(rect).into(),
        material: materials.add(ColorMaterial::default()),
        transform: Transform::from_scale(Vec3::splat(256.))
            .with_translation(Vec3::new(512., 0., 0.)),
        ..default()
    });
}
