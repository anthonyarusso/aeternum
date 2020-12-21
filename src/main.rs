use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// import 'custom_plugins' as a module
mod custom_plugins;

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(custom_plugins::main_menu::MainMenuPlugin)
        .init_resource::<BackgroundMaterials>()
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
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

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    background_materials: Res<BackgroundMaterials>,
) {
    commands
        // 2d camera
        .spawn(CameraUiBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: background_materials.alphadark.clone(), 
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
            style: Style {
                padding: Rect::all(Val::Auto),
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/SourceSansPro-Regular.ttf"),
                style: TextStyle {
                    font_size: 24.0,
                    color: Color::YELLOW,
                    ..Default::default()
                },
            },
            ..Default::default()
            })
            .with(FpsText);
        });
}