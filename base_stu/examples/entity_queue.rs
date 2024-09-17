use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    prelude::{Commands, Component, Entity, Mesh, Query, Rectangle, ResMut},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    DefaultPlugins,
};

#[derive(Component, Debug)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, receive)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Name("霖霖".to_string()));
    commands.spawn(Name("lilin".to_string()));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
        material: materials.add(ColorMaterial::default()),
        ..Default::default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
        material: materials.add(ColorMaterial::default()),
        ..Default::default()
    });
}

fn receive(query: Query<(Entity, &Name)>, mut query2: Query<(Entity, &Mesh2dHandle)>) {
    let mut i: u32 = 1;
    for (entity, name) in &query {
        assert_eq!(entity.index(), i);
        i += 1;
    }
    for (entity, mesh) in &mut query2.iter() {
        assert_eq!(entity.index(), i);
        println!("entity: {:?}", entity);
        i += 1;
    }
}
