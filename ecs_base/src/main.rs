use bevy::{
    app::{App, Startup, Update},
    prelude::{
        Bundle, Commands, Component, Entity, Event, EventReader, EventWriter, Query, ResMut,
        Resource,
    },
};

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, start_up)
        .add_systems(
            Update,
            (
                level_up,
                print_health,
                print_xp,
                my_system_res,
                my_system_bundle,
            ),
        )
        .add_event::<LevelUpEvent>()
        .run();
}
#[derive(Component, Debug)]
struct Xp(u32);

#[derive(Component, Debug)]
struct Health {
    current: u32,
    max: u32,
}

fn start_up(mut commands: Commands) {
    commands.spawn(Xp(999));
    commands.spawn(Xp(10000));
    commands.spawn(Health {
        current: 10,
        max: 1000,
    });
    commands.spawn((
        Xp(99999),
        Health {
            current: 9,
            max: 999999,
        },
    ));
    commands.spawn((
        Xp(9999900),
        Health {
            current: 9,
            max: 999999,
        },
    ));
    // 资源
    commands.insert_resource(GoalsReached {
        main_goal: true,
        bonus: 5,
    });
    // 捆绑包
    commands.spawn(PlayerBundle {
        xp: Xp(55555),
        health: Health {
            current: 5555,
            max: 555555,
        },
    });
}

fn level_up(
    // We want to access the Xp and Health data:
    mut query: Query<(&mut Xp, &mut Health)>,
) {
    // process all relevant entities
    for (mut xp, mut health) in query.iter_mut() {
        if xp.0 > 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
        }
    }
}

fn print_xp(query: Query<(&mut Xp, &mut Health)>) {
    for (xp, _health) in query.iter() {
        println!("xp = {}", xp.0);
    }
}
fn print_health(query: Query<&Health>) {
    for health in query.iter() {
        println!("{:?}", health);
    }
}

#[derive(Resource, Debug)]
struct GoalsReached {
    main_goal: bool,
    bonus: u32,
}

fn my_system_res(
    // these will panic if the resources don't exist
    mut goals: ResMut<GoalsReached>,
    // use Option if a resource might not exist
    // mut fancy: Option<ResMut<MyFancyResource>>,
) {
    // if let Some(fancy) = &mut fancy {
    //     // TODO: do things with `fancy`
    // }
    // TODO: do things with `goals` and `other`
    println!("{:?}", goals);
}
#[derive(Bundle)]
struct PlayerBundle {
    xp: Xp,
    health: Health,
}

fn my_system_bundle(query: Query<(&Xp, &Health)>) {
    for (xp, health) in query.iter() {
        println!("{:?}, {:?}", xp, health);
    }
}
#[derive(Event)]
struct LevelUpEvent(Entity);

fn player_level_up(mut ev_level_up: EventWriter<LevelUpEvent>, query: Query<(Entity, &Xp)>) {
    for (entity, xp) in query.iter() {
        if xp.0 > 1000 {
            ev_level_up.send(LevelUpEvent(entity));
        }
    }
}

fn debug_level_ups(mut ev_level_up: EventReader<LevelUpEvent>) {
    for ev in ev_level_up.read() {
        println!("Entity {:?} leveled up!", ev.0);
    }
}
