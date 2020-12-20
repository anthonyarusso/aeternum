use bevy::prelude::*;

fn add_people(commands: &mut Commands) {
    commands
    .spawn((Person, Name("Elaina Proctor".to_string())))
    .spawn((Person, Name("Renzo Hume".to_string())))
    .spawn((Person, Name("Zayna Nieves".to_string())));
}


struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if the timer hasn't finished yet, we return
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // the reason we call from_seconds with the ture flag is to make the timer repeat itself
        app
            .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());

    }
}

struct Person;
struct Name(String);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();

}