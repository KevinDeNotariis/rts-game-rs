use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{Collider, RapierPickable},
    render::ColliderDebugColor,
};

use crate::{game_state::GameState, terrain::TerrainResource};

use super::{
    assets::RomeBuildingsAssets,
    factory::{RomeBuildingType, RomeBuildingsFactoryResource},
};

pub struct RomeBuildingsUIPlugin;

impl Plugin for RomeBuildingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UserActionState>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    place_or_cancel_building.run_if(in_state(UserActionState::PlacingBuilding)),
                    move_building_getting_placed.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum UserActionState {
    #[default]
    None,
    PlacingBuilding,
    BuildingSelected,
    UnitsSelected,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RomeBuildingsUIFrame;

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Name::new("RomeBuildingUI"),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                align_self: AlignSelf::End,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![(
                Button,
                RomeBuildingsUIFrame,
                Node {
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![
                    Text::new("Spawn Building"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ]
            )],
        ))
        .observe(spawn_holo);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingGettingPlaced;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingSelected;

fn spawn_holo(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    buildings_factory_res: Res<RomeBuildingsFactoryResource>,
    terrain_resource: Res<TerrainResource>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RomeBuildingsUIFrame>)>,
    mut next_user_action_state: ResMut<NextState<UserActionState>>,
    current_user_action_state: Res<State<UserActionState>>,
) -> Result {
    if current_user_action_state.get() == &UserActionState::PlacingBuilding {
        return Ok(());
    }

    let hit = terrain_resource.cursor_projection;

    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let factory = buildings_factory_res.factory;
                factory.spawn_holo(
                    &mut commands,
                    RomeBuildingType::Cottage,
                    Vec3::new(hit.x, 0.0, hit.y),
                );
                next_user_action_state.set(UserActionState::PlacingBuilding);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
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

fn move_building_getting_placed(
    mut building_q: Query<&mut Transform, With<BuildingGettingPlaced>>,
    terrain_resource: Res<TerrainResource>,
) -> Result {
    if let Ok(mut building) = building_q.single_mut() {
        let hit = terrain_resource.cursor_projection;

        let current_height = building.translation.y;
        building.translation = Vec3::new(
            hit.x,
            current_height, // or hit_position.y + offset
            hit.y,
        )
    }

    Ok(())
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
