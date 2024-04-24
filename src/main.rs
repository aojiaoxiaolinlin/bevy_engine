use bevy::{app::App, DefaultPlugins};
use bevy_game::HelloPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins,HelloPlugin))
        .run();
}

