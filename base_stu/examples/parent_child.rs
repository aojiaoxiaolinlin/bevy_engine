use bevy::{
    color::palettes::css::{BLUE, RED},
    prelude::*,
};
#[derive(Default)]
struct Count(f32);
fn main() {
    // 全局变量
    let mut count = Count::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            move |commands: Commands,
                  meshes: ResMut<Assets<Mesh>>,
                  materials: ResMut<Assets<ColorMaterial>>| {
                update_spawn(commands, meshes, materials, &mut count);
            },
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    // 创建父实体
    commands
        .spawn((
            Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
            Transform::from_scale(Vec3::splat(128.))
                .with_translation(Vec3::new(0., 128., 0.))
                .with_rotation(Quat::from_rotation_z(60.0_f32.to_radians())),
        ))
        .with_children(|child_builder| {
            // 创建子实体1
            child_builder.spawn((
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(materials.add(Color::from(RED))),
                Transform::from_translation(Vec3::new(1., 0., 0.)),
            ));

            // 创建子实体2
            child_builder.spawn((
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(materials.add(Color::from(BLUE))),
                Transform::from_translation(Vec3::new(-1., 0., 0.)),
            ));
        });
}

fn update_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    count: &mut Count,
) {
    count.0 += 1.0;
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_translation(Vec3::new(-count.0, 0., 0.)),
    ));
}
