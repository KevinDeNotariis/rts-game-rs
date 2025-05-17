use bevy::prelude::*;

use crate::{
    components::buildings::{BuildingGettingPlaced, BuildingSelected},
    factions::rome::buildings::models::RomeBuildingType,
    states::UserActionState,
    terrain::TerrainResource,
};

use super::{assets::RomeBuildingsAssets, factory::RomeBuildingsFactoryResource};

pub struct RomeBuildingsLifecyclePlugin;

impl Plugin for RomeBuildingsLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            place_or_cancel_building.run_if(in_state(UserActionState::PlacingBuilding)),
        );
    }
}

fn place_or_cancel_building(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    building: Query<Entity, With<BuildingGettingPlaced>>,
    mut next_user_action_state: ResMut<NextState<UserActionState>>,
    terrain_resource: Res<TerrainResource>,
    buildings_factory_res: Res<RomeBuildingsFactoryResource>,
    rome_building_asset: Res<RomeBuildingsAssets>,
) -> Result {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let hit = terrain_resource.cursor_projection;

        println!("Placed!");
        let factory = buildings_factory_res.factory;
        factory
            .spawn(
                &mut commands,
                &rome_building_asset,
                RomeBuildingType::Cottage,
                Vec3::new(hit.x, 0.0, hit.y),
            )
            .observe(highlight_building);
    }
    if mouse_button_input.just_pressed(MouseButton::Right) || key.just_pressed(KeyCode::Escape) {
        println!("Canceled!");
        commands.entity(building.single()?).despawn();
        next_user_action_state.set(UserActionState::None);
    }

    Ok(())
}

fn highlight_building(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    user_action_state: Res<State<UserActionState>>,
    mut next_user_action_state: ResMut<NextState<UserActionState>>,
    building_selected: Query<Entity, With<BuildingSelected>>,
    entities: Query<Entity>,
) -> Result {
    if user_action_state.get() == &UserActionState::PlacingBuilding {
        return Ok(());
    }
    if user_action_state.get() == &UserActionState::BuildingSelected {
        let previous_selected = entities.get(building_selected.single()?)?;
        commands
            .entity(previous_selected)
            .remove::<BuildingSelected>()
            .remove::<MeshMaterial3d<StandardMaterial>>();
    }

    println!("Clicked Building!");
    let entity = entities.get(_click.target())?;

    let highlight_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.5), // Highlight color
        emissive: LinearRgba::new(0.5, 0.5, 0.2, 0.3), // Add glow effect
        ..Default::default()
    });
    commands
        .entity(entity)
        .insert((MeshMaterial3d(highlight_material), BuildingSelected));

    next_user_action_state.set(UserActionState::BuildingSelected);

    Ok(())
}
