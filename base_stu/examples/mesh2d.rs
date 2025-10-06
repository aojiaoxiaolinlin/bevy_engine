use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{PURPLE, RED, WHITE, YELLOW},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component, Default)]
#[require(Visibility, Transform)]
struct Parent;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200.0, 100.0))),
        MeshMaterial2d(color_materials.add(ColorMaterial::from_color(RED))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.0))),
        MeshMaterial2d(color_materials.add(ColorMaterial::from_color(WHITE))),
    ));
}

fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut i: Local<bool>,
) {
    if *i {
        return;
    }
    *i = true;
    commands
        .spawn((
            Parent,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.2)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(200.0, 100.0))),
                MeshMaterial2d(color_materials.add(ColorMaterial::from_color(YELLOW))),
                Transform::from_translation(Vec3::new(100.0, 0.0, 0.1))
                    .with_rotation(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), PI / 2.0)),
            ));
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(100.0))),
                MeshMaterial2d(color_materials.add(ColorMaterial::from_color(PURPLE))),
                Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            ));
        });
}
