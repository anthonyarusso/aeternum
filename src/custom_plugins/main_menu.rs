/* A custom plugin to implement menu screens within Bevy */
use bevy::{
    app::AppExit,
    audio,
    prelude:: *,
};

use std::fmt;

use crate::custom_resources::materials;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(audio::AudioPlugin)
            .add_resource(State::new(AppState::MainMenu))
            .init_resource::<materials::ButtonMaterials>()
            .init_resource::<GlobalCounters>()
            .init_resource::<MenuOptions>()
            .init_resource::<StateHistory>()
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .on_state_enter(STAGE, AppState::MainMenu, setup_menu.system())
            .on_state_update(STAGE, AppState::MainMenu, menu.system())
            .on_state_exit(STAGE, AppState::MainMenu, cleanup_menu.system())
            .on_state_enter(STAGE, AppState::PauseMenu, setup_pause_menu.system())
            .on_state_update(STAGE, AppState::PauseMenu, pause_menu.system())
            .on_state_exit(STAGE, AppState::PauseMenu, cleanup_pause_menu.system())
            .on_state_enter(STAGE, AppState::SettingsMenu, setup_settings_menu.system())
            .on_state_update(STAGE, AppState::SettingsMenu, settings_menu.system())
            .on_state_exit(STAGE, AppState::SettingsMenu, cleanup_settings_menu.system())           .on_state_enter(STAGE, AppState::InGame, setup_game.system())
            .on_state_update(STAGE, AppState::InGame, movement.system())
            .on_state_update(STAGE, AppState::InGame, change_color.system())
            .on_state_exit(STAGE, AppState::InGame, cleanup_game.system());
    }
}

const STAGE: &str = "app_state";

#[derive(Clone, Copy, Debug)]
enum AppState {
    MainMenu,
    PauseMenu,
    SettingsMenu,
    InGame,
}

const HISTORY_SIZE: usize = 5;

struct StateHistory {
    history: [AppState; HISTORY_SIZE],
    count: usize,
}

impl FromResources for StateHistory {
    fn from_resources(_resources: &Resources) -> Self {
        StateHistory {
            history: [AppState::MainMenu; HISTORY_SIZE],
            count: 0,
        }
    }
}

impl fmt::Debug for StateHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateHistory")
         .field("history", &self.history)
         .field("count", &self.count)
         .finish()
    }
}

impl StateHistory {
    fn clear(&mut self) {
        for i in 0..HISTORY_SIZE {
            self.history[i] = AppState::MainMenu;
        }
        self.count = 0;
    }
    fn push(&mut self, state: AppState) {
        self.history[0] = state;
        let temp_history = self.history.clone();
        // shift history values down
        for i in 0..(HISTORY_SIZE-1) {
            self.history[i+1] = temp_history[i];
        }
        if self.count < 5 {
            self.count += 1;
        }
    }
    fn prev(&mut self) -> AppState {
        if self.count <= 0 {
            eprintln!("StateHistory.prev() Error: No previous history.");
            AppState::MainMenu
        } else {
            let temp_history = self.history.clone();
            // shift up the history values by one
            for i in 0..(HISTORY_SIZE-1) {
                self.history[i] = temp_history[i+1];
            }
            temp_history[0]
        }
    }
}

pub struct MenuOptions {
    main: [&'static str; 5],
    pause: [&'static str; 4],
    settings: [&'static str; 6],
}

impl FromResources for MenuOptions {
    fn from_resources(_resources: &Resources) -> Self {
        MenuOptions {
            main: [
                "Play Game",
                "New Game",
                "Settings",
                "Credits",
                "Exit",
            ],
            pause: [
                "Resume Game",
                "Settings",
                "Main Menu",
                "Exit Game",
            ],
            settings: [
                "Accessibility",
                "Audio",
                "Controls",
                "Graphics",
                "Language",
                "Previous Menu",
            ],
        }
    }
}

pub struct GlobalCounters {
    audio_counter: u32,
}

impl FromResources for GlobalCounters {
    fn from_resources(_resources: &Resources) -> Self {
        GlobalCounters {
            audio_counter: 0,
        }
    }
}

struct MainMenuEntity {
    main_entity: Entity,
}

struct SettingsMenuEntity {
    main_entity: Entity,
}

struct PauseMenuEntity {
    main_entity: Entity,
}

struct GameData {
    sprite_entity: Entity,
}

fn setup_pause_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<materials::ButtonMaterials>,
    menu_options: Res<MenuOptions>,
) {
    commands
        .spawn(CameraUiBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..menu_options.pause.len() {
                parent.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Px(25.0)),
                        justify_content: JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: menu_options.pause[i].to_string(),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                }); 
            } // end for-loop
        });
    commands.insert_resource(PauseMenuEntity {
        main_entity: commands.current_entity().unwrap(),
    });
}

fn setup_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<materials::ButtonMaterials>,
    audio: Res<Audio>,
    mut counter_res: ResMut<GlobalCounters>,
    menu_options: Res<MenuOptions>,
) {
    let main_menu_music = asset_server.load("audio/music/Sad_Italian_Song.mp3");
    commands
        .spawn(CameraUiBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..menu_options.main.len() {
                parent.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Px(25.0)),
                        justify_content: JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: menu_options.main[i].to_string(),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                }); 
            } // end for-loop
        });
    commands.insert_resource(MainMenuEntity {
        main_entity: commands.current_entity().unwrap(),
    });
    if counter_res.audio_counter == 0 {
        audio.play(main_menu_music.clone());
        counter_res.audio_counter += 1;
    }
}

fn setup_settings_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<materials::ButtonMaterials>,
    menu_options: Res<MenuOptions>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..menu_options.settings.len() {
                parent.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Px(25.0)),
                        justify_content: JustifyContent::Center,
                        align_items:AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            value: menu_options.settings[i].to_string(),
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(1.0, 1.0, 1.0),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    });
                }); 
            } // end for-loop
        });
    commands.insert_resource(SettingsMenuEntity {
        main_entity: commands.current_entity().unwrap(),
    });
}

fn menu(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<materials::ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    menu_options: Res<MenuOptions>,
) {
    let button_click = asset_server.load("audio/sounds/click.mp3");
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                audio.play(button_click.clone());
                if text.value == menu_options.main[0].to_string() {
                    state.set_next(AppState::InGame).unwrap();
                } else if text.value == menu_options.main[1].to_string() {
                    state.set_next(AppState::InGame).unwrap();
                } else if text.value == menu_options.main[2].to_string() {
                    state.set_next(AppState::SettingsMenu).unwrap()
                } else if text.value == menu_options.main[3].to_string() {
                    println!("Made by ur mom, lol");
                } else if text.value == menu_options.main[4].to_string() {
                    app_exit_events.send(AppExit);
                } else {
                    println!("Main Menu Text");
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn pause_menu(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<materials::ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    menu_options: Res<MenuOptions>,
) {
    let button_click = asset_server.load("audio/sounds/click.mp3");
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                audio.play(button_click.clone());
                if text.value == menu_options.pause[0].to_string() {
                    state.set_next(AppState::InGame).unwrap();
                } else if text.value == menu_options.pause[1].to_string() {
                    state.set_next(AppState::SettingsMenu).unwrap();
                } else if text.value == menu_options.pause[2].to_string() {
                    state.set_next(AppState::MainMenu).unwrap();
                } else if text.value == menu_options.pause[3].to_string() {
                    app_exit_events.send(AppExit);
                } else {
                    println!("Mama mia!");
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn settings_menu(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<materials::ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    menu_options: Res<MenuOptions>,
    mut history: ResMut<StateHistory>,
) {
    let button_click = asset_server.load("audio/sounds/click.mp3");
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                audio.play(button_click.clone());
                if text.value == menu_options.settings[0].to_string() {
                    println!("Display Accessibility Settings");
                } else if text.value == menu_options.settings[5].to_string() {
                    state.set_next(history.prev()).unwrap();
                } else {
                    println!("Clicky clack");
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn cleanup_settings_menu(
    commands: &mut Commands,
    menu_data: Res<SettingsMenuEntity>,
    mut history: ResMut<StateHistory>,
) {
    history.push(AppState::SettingsMenu);
    commands.despawn_recursive(menu_data.main_entity);
}

fn cleanup_pause_menu(
    commands: &mut Commands,
    menu_data: Res<PauseMenuEntity>,
    mut history: ResMut<StateHistory>,
) {
    history.push(AppState::PauseMenu);
    commands.despawn_recursive(menu_data.main_entity);
}

fn cleanup_menu(commands: &mut Commands,
    menu_data: Res<MainMenuEntity>,
    mut history: ResMut<StateHistory>,
) {
    history.push(AppState::MainMenu);
    commands.despawn_recursive(menu_data.main_entity);
}

fn setup_game(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut history: ResMut<StateHistory>,
) {
    history.clear();
    let texture_handle = asset_server.load("images/main_menu/ancient_rome.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
    commands
        .insert_resource(GameData {
           sprite_entity: commands.current_entity().unwrap(), 
        });
}

fn cleanup_game(commands: &mut Commands, game_data: Res<GameData>) {
    commands.despawn_recursive(game_data.sprite_entity);
}

const SPEED: f32 = 900.0;
fn movement(
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::default();
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::Escape) {
           state.set_next(AppState::PauseMenu).unwrap(); 
        }

        if direction != Vec3::default() {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

fn change_color(
    time: Res<Time>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    query: Query<&Handle<ColorMaterial>, With<Sprite>>,
) {
    for handle in query.iter() {
        let material = assets.get_mut(handle).unwrap();
        material
            .color
            .set_b((time.seconds_since_startup() * 5.0).sin() as f32 + 2.0);
    }
}
