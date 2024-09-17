use bevy::{
    app::{App, Startup, Update},
    prelude::{Bundle, Commands, Component, Entity, IntoSystemConfigs, Query, With},
};

#[derive(Component, Default)]
struct Person {
    name: String,
}
#[derive(Component, Default)]
struct Name {
    name: String,
}
#[derive(Bundle, Default)]
struct PersonBundle {
    name: Name,
    person: Person,
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (add_person, query_component).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PersonBundle {
        name: Name {
            name: "name".to_string(),
        },
        person: Person {
            name: "person".to_string(),
        },
    });
}
fn add_person(mut commands: Commands, query: Query<Entity, With<Name>>) {
    // query.iter().for_each(|entity| {
    //     commands.entity(entity).insert(Person {
    //         name: "person2".to_string(),
    //     });
    // });
    commands.spawn(Person {
        name: "person3".to_string(),
    });
}
fn query_component(query: Query<&Name>, query_person: Query<&Person>) {
    query.iter().for_each(|name| {
        println!("name:{}", name.name);
    });
    query_person.iter().for_each(|person| {
        println!("person:{}", person.name);
    });
}
