use bevy::{
    app::{App, Startup}, asset::AssetServer, math::Vec3, prelude::{Camera2dBundle, Commands, Res}, sprite::SpriteBundle, transform::components::Transform, DefaultPlugins
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
}
