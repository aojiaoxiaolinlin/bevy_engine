use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    color::Color,
    math::Vec3,
    pbr::AmbientLight,
    prelude::{Bundle, Camera3dBundle, Commands, Component, Res},
    render::camera::ClearColor,
    scene::SceneBundle,
    transform::components::Transform,
    DefaultPlugins,
};
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.0);

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_spaceship);
    }
}
#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.85,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_spaceship))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 60.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}
fn spawn_spaceship(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..Default::default()
        },
    });
}
