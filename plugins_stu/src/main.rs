use bevy::{app::App, DefaultPlugins};
use plugins_stu::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
