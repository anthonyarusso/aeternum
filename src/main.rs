use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    // ecs::SystemStage,
    prelude::*,
    // utils::Duration,
};

// import the module 'custom_plugins.rs'
mod custom_plugins;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(custom_plugins::main_menu::MainMenuPlugin)
        .init_resource::<ButtonMaterials>()
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}