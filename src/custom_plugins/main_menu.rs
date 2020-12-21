use bevy::{
    prelude::*,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system());
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
                size: Size::new(Val::Percent(35.0), Val::Percent(60.0)),
                align_items: AlignItems::Baseline,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            material: background_materials.alphadark.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
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
                        value: "Settings".to_string(),
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
                        value: "Credits".to_string(),
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
                        value: "Load Game".to_string(),
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
                        value: "Play Game".to_string(),
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
    alphadark: Handle<ColorMaterial>,
}

impl FromResources for BackgroundMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        BackgroundMaterials {
            alphadark: materials.add(Color::rgba(0.3, 0.3, 0.3, 0.7).into()),
        }
    }
}
