use bevy::{
    app::{App, Startup, Update},
    prelude::{Commands, Component, Entity, IntoSystemConfigs, Local, Query, SystemSet, With},
};
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MySystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MySystemSet2;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

#[derive(Default)]
struct MyLocal {
    name: String,
}

fn main() {
    App::new()
        // 添加系统
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (
                hello_world.in_set(MySystemSet),
                play.after(MySystemSet).in_set(MySystemSet2),
                (update_people, greet_people).chain().after(MySystemSet2), // 通过chain()方法将保证执行顺序按照编写顺序运行
                shay.before(MySystemSet2).after(MySystemSet),
                set_my_local1,
                set_my_local2,
            ),
        )
        .run();
}

fn hello_world() {
    println!("bevy 我来了");
}

fn play() {
    println!("bevy 我在玩");
}

fn shay() {
    println!("bevy 我在说");
}

fn add_people(mut commands: Commands) {
    // commands将实体添加到世界
    commands.spawn((Person, Name("霖霖".to_string())));
    commands.spawn((Person, Name("lilin".to_string())));
}

// 该函数将查询所有具有Person和Name组件的实体，并打印它们的名称
fn greet_people(query: Query<(Entity, &Name), With<Name>>) {
    for (entity, name) in &query {
        println!("entity: {:?}", entity);
        println!("你好, {}", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0.push('!');
    }
}
/// `Local`在每个`System`中都是唯一的，因此可以用来存储系统的局部状态
fn set_my_local1(mut my_local: Local<MyLocal>) {
    println!("my_local.name1: {}", my_local.name);
    my_local.name = "lilin1".to_string();
    println!("my_local.name1 after set: {}", my_local.name);
}

fn set_my_local2(mut my_local: Local<MyLocal>) {
    println!("my_local.name2: {}", my_local.name);
    my_local.name = "lilin2".to_string();
    println!("my_local.name2 after set: {}", my_local.name);
}
