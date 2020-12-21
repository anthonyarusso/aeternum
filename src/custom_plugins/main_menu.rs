use bevy::{
    prelude::*,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<BackgroundMaterials>()
            .init_resource::<ButtonMaterials>()
            .init_resource::<MainMenuButtons>()
            .add_startup_system(setup.system())
            .add_system(button_system.system());
    }
}

pub const BUTTON_COUNT_MAIN: usize = 4;

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    main_menu_buttons: Res<MainMenuButtons>,
    button_materials: Res<ButtonMaterials>,
    background_materials: Res<BackgroundMaterials>,
) {
    // load text from the MainMenuButtons resource into an iterable array
    let button_texts: [&str; BUTTON_COUNT_MAIN] =
        [main_menu_buttons.new_game.text,
        main_menu_buttons.load_game.text,
        main_menu_buttons.credits.text,
        main_menu_buttons.settings.text];
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(128.0)),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(40.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: background_materials.alpha.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Aeternum".to_string(),
                    font: asset_server.load("fonts/TimesNewRoman.ttf"),
                    style: TextStyle {
                        font_size: 114.0,
                        color: Color::GOLD,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(35.0), Val::Percent(60.0)),
                align_items: AlignItems::Baseline,
                /* Buttons are added from bottom-to-top. ColumnReverse displays
                the buttons in reverse order making them appear
                in the same order as button_texts. */
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            material: background_materials.alphadark.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Loop through each button and add the correct text from button_texts
            for i in 0..BUTTON_COUNT_MAIN {
               parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                    margin: Rect {
                        left: Val::Px(20.0),
                        ..Default::default()
                    }, 
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                text: Text {
                        value: button_texts[i].to_string(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default() 
                });
            }); 
            }
        });
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

struct BackgroundMaterials {
    alpha: Handle<ColorMaterial>,
    alphadark: Handle<ColorMaterial>,
}

impl FromResources for BackgroundMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        BackgroundMaterials {
            alpha: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            alphadark: materials.add(Color::rgba(0.3, 0.3, 0.3, 0.7).into()),
        }
    }
}

struct MenuButton {
    text: &'static str,
    // implement a function pointer here so we can pass unique functions to each button
}

impl MenuButton {
    fn new(text: &'static str) -> MenuButton {
        MenuButton {
            text: text,
        }
    }
}

struct MainMenuButtons {
    new_game: MenuButton,
    load_game: MenuButton,
    credits: MenuButton,
    settings: MenuButton,
}

impl FromResources for MainMenuButtons {
    fn from_resources(_resources: &Resources) -> Self {
        MainMenuButtons {
            new_game: MenuButton::new("New Game"),
            load_game: MenuButton::new("Load Game"),
            credits: MenuButton::new("Credits"),
            settings: MenuButton::new("Settings"),
        }
    }
}
// to-do: make this a Resource to properly implement 'button_texts' as a global

fn button_system (
    main_menu_buttons: Res<MainMenuButtons>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let button_text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                let x = button_text.value.as_str();
                match x {
                    _ if x == main_menu_buttons.new_game.text => println!("New game"),
                    _ if x == main_menu_buttons.load_game.text => println!("Loading game..."),
                    _ if x == main_menu_buttons.credits.text => println!("Made by ur mom, lol"),
                    _ if x == main_menu_buttons.settings.text => println!("Settings"),
                    _ => println!("UNKNOWN BUTTON PRESSED"),
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