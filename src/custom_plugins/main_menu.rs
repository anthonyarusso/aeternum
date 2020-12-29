use bevy::{
    prelude:: *,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system());
    }
}

fn setup(
    // commands: &mut Commands,
) {
    println!("MainMenuPlugin successfully loaded.");
}