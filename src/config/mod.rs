use bevy::prelude::*;

use crate::config::{camera::CameraConfigPlugin, terrain::TerrainConfigPlugin};

pub mod camera;
pub mod terrain;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraConfigPlugin, TerrainConfigPlugin));
    }
}
