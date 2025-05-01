use bevy::prelude::*;

pub mod rome;

pub struct FactionsPlugin;

impl Plugin for FactionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(rome::RomePlugin);
    }
}
