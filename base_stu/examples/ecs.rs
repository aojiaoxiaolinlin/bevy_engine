use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::Children,
        system::{Commands, Query},
    },
};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component, Clone)]
struct Person(u8);

#[derive(Component)]
struct Name(String);

fn setup(mut commands: Commands) {
    commands.spawn(Person(1)).with_children(|parent| {
        parent.spawn(Name("霖霖".to_string()));
        parent.spawn(Name("小明".to_string()));
    });
}

fn update(query: Query<(Entity, &Person, &Children)>) {
    let mut tmp = None;
    for (entity, person, children) in query.iter() {
        println!("{} is a person named", person.0);
        println!("Entity: {:?}", entity.index());
        for child in children.iter() {
            if tmp.is_none() {
                println!("Entity: {:?}", child.index());
                tmp = Some(child);
            } else {
                println!("Entity: {:?}", child.index());
                println!("相等吗: {:?}", tmp == Some(child));
            }
        }
    }
}
