/* A custom plugin to implement menu screens within Bevy */
use bevy::{
    app::AppExit,
    audio,
    prelude:: *,
};

use crate::custom_resources::materials;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(audio::AudioPlugin)
            .add_resource(State::new(AppState::Menu))
            .init_resource::<materials::ButtonMaterials>()
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .on_state_enter(STAGE, AppState::Menu, setup_menu.system())
            .on_state_update(STAGE, AppState::Menu, menu.system())
            .on_state_exit(STAGE, AppState::Menu, cleanup_menu.system())
            .on_state_enter(STAGE, AppState::InGame, setup_game.system())
            .on_state_update(STAGE, AppState::InGame, movement.system())
            .on_state_update(STAGE, AppState::InGame, change_color.system())
            .on_state_exit(STAGE, AppState::InGame, cleanup_game.system());
    }
}

const STAGE: &str = "app_state";

#[derive(Clone)]
enum AppState {
    Menu,
    InGame,
}

struct MainMenuEntity {
    main_entity: Entity,
}

struct GameData {
    sprite_entity: Entity,
}

fn setup_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<materials::ButtonMaterials>,
    audio: Res<Audio>,
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
                        value: "Resume Game".to_string(),
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
                        value: "Mama mia!".to_string(),
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
                        value: "Exit Game".to_string(),
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
        });
    commands.insert_resource(MainMenuEntity {
        main_entity: commands.current_entity().unwrap(),
    });
    audio.play(main_menu_music.clone());
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
) {
    let button_click = asset_server.load("audio/sounds/click.mp3");
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                audio.play(button_click.clone());
                if text.value == "Resume Game".to_string() {
                    state.set_next(AppState::InGame).unwrap();
                } else if text.value == "Exit Game".to_string() {
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

fn cleanup_menu(commands: &mut Commands, menu_data: Res<MainMenuEntity>) {
    commands.despawn_recursive(menu_data.main_entity);
}

fn setup_game(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
           state.set_next(AppState::Menu).unwrap(); 
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
