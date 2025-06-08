use bevy::prelude::*;

pub struct TerrainConfigPlugin;

impl Plugin for TerrainConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TerrainConfig>()
            .insert_resource(TerrainConfig { x: 5., y: 5. });
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct TerrainConfig {
    pub x: f32,
    pub y: f32,
}
