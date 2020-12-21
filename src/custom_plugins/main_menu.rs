use bevy::{
    prelude::*,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<BackgroundMaterials>()
            .init_resource::<ButtonMaterials>()
            .add_startup_system(setup.system())
            .add_system(button_system.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    background_materials: Res<BackgroundMaterials>,
) {
    let button_texts: [&str; 4] = ["New Game", "Load Game", "Credits", "Settings"];
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(72.0)),
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
                        font_size: 60.0,
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
            for i in 0..4 {
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

fn button_system (
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
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