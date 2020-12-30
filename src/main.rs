use bevy::{
    // app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    // ecs::SystemStage,
    prelude::*,
    // utils::Duration,
};

// import the custom plugins and resources
mod custom_plugins { pub mod main_menu; }
mod custom_resources { pub mod materials; }

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(custom_plugins::main_menu::MainMenuPlugin)
        .run();
}