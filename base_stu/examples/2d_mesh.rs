use bevy::{
    app::{App, Startup},
    asset::{Assets, Handle},
    color::{
        palettes::{
            css::{BLUE, GREEN, PURPLE, RED},
            tailwind::BLUE_500,
        },
        ColorToComponents,
    },
    math::Vec3,
    prelude::{
        Bundle, Camera2dBundle, Commands, Mesh, Msaa, Rectangle, ResMut, SpatialBundle, Transform,
        Triangle2d,
    },
    render::render_asset::RenderAssetUsages,
    sprite::{ColorMaterial, ColorMesh2dBundle, Mesh2dHandle},
    DefaultPlugins,
};
use wgpu::PrimitiveTopology;

#[derive(Debug, Bundle)]
struct CustomMesh2DBundle {
    mesh: Mesh2dHandle,
    // 其他可能需要的组件，比如材质、变换等
    spatial: SpatialBundle,
    material: Handle<ColorMaterial>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa::Sample8)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    // let mut rect = Mesh::from(Rectangle::default());
    // rect.insert_attribute(
    //     Mesh::ATTRIBUTE_COLOR,
    //     vec![
    //         RED.to_f32_array(),
    //         GREEN.to_f32_array(),
    //         BLUE_500.to_f32_array(),
    //         PURPLE.to_f32_array(),
    //     ],
    // );
    // commands.spawn(CustomMesh2DBundle {
    //     mesh: meshes.add(rect).into(),
    //     material: materials.add(ColorMaterial::default()),
    //     spatial: SpatialBundle {
    //         transform: Transform::from_scale(Vec3::splat(256.)),
    //         ..Default::default()
    //     },
    // });

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1., -1., 0.0], [3., -1., 0.0], [2.0, 1., 0.0]],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            RED.to_f32_array(),
            GREEN.to_f32_array(),
            BLUE.to_f32_array(),
        ],
    );

    commands.spawn(ColorMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::default()),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(Vec3::splat(400.)),
        ..Default::default()
    });
}
