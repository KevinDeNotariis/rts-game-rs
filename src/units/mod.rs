use bevy::prelude::*;

pub mod rome;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rome::RomeUnitsPlugin);
    }
}
