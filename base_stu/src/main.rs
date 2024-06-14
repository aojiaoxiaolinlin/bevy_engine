use bevy::{
    app::{App, Startup, Update},
    prelude::{Commands, Component, IntoSystemConfigs, Query, With},
};

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
fn main() {
    App::new()
        // 添加系统
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (
                hello_world,
                (update_people, greet_people).chain(), // 通过chain()方法将保证执行顺序按照编写顺序运行
            ),
        )
        .run();
}

fn hello_world() {
    println!("bevy 我来了");
}

fn add_people(mut commands: Commands) {
    // commands将实体添加到世界
    commands.spawn((Person, Name("霖霖".to_string())));
    commands.spawn((Person, Name("艺洁".to_string())));
}

// 该函数将查询所有具有Person和Name组件的实体，并打印它们的名称
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("你好, {}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0.push_str("!");
    }
}
