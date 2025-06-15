use bevy::prelude::*;

use crate::units::Selected;

// De-select previously selected units by:
// 1. Removing Selected component on the unit
// 2. Removing Selected component in the unitSelector and make it Hidden
pub fn remove_selection(
    commands: &mut Commands,
    selected_units: impl IntoIterator<Item = Entity>,
    selectors: impl IntoIterator<Item = Entity>,
) {
    for entity in selected_units {
        commands.entity(entity).remove::<Selected>();
    }
    for entity in selectors {
        commands.entity(entity).remove::<Selected>();
        commands.entity(entity).insert(Visibility::Hidden);
    }
}
