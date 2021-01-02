/* Defines custom materials. */

use bevy::{
    prelude::*
};

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
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

pub struct BackgroundMaterials {
    pub alpha: Handle<ColorMaterial>,
    pub alpha_red: Handle<ColorMaterial>,
    pub image_main: Handle<ColorMaterial>,
}

impl FromResources for BackgroundMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();
        BackgroundMaterials {
            alpha: materials.add(Color::NONE.into()),
            alpha_red: materials.add(Color::rgba(255.0, 0.0, 0.0, 0.6).into()),
            image_main: materials
                .add(asset_server.load("images/main_menu/ancient_rome_trees.png").into()),
        }
    }
}