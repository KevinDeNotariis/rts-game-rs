use bevy::{platform::collections::HashMap, prelude::*};

use crate::base::factions::Factions;

#[derive(Reflect, Clone, Copy)]
pub struct BuildingsConfig {
    size: f32,
}

// #[derive(Reflect, Clone, Copy)]
// pub struct BuildingsFactory {
//     pub config: HashMap<Factions, HashMap<Buildings>> BuildingsConfig,
// }

// #[derive(Resource, Reflect)]
// #[reflect(Resource)]
// pub struct BuildingsFactoryResource {
//     pub factory: BuildingsFactory,
// }
