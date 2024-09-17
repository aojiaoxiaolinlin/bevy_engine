use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    math::Vec3,
    prelude::{Camera2dBundle, Changed, Commands, Query, Res},
    sprite::SpriteBundle,
    transform::components::Transform,
    window::Window,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_window_info)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        // 不支持jpg格式
        // texture: asset_server.load("lu_na.jpg"),
        texture: asset_server.load("huo.png"),
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        // 不支持jpg格式
        // texture: asset_server.load("lu_na.jpg"),
        texture: asset_server.load("huo.png"),
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0))
            .with_translation(Vec3::new(256.0, 0.0, 0.0)),
        ..Default::default()
    });
}

fn update_window_info(window: Query<&Window, Changed<Window>>) {
    if let Ok(window) = window.get_single() {
        println!("window size: {:?}", window.physical_width());
        println!("window size: {:?}", window.physical_height());
    }
}
