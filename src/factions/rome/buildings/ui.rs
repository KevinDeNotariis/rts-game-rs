use bevy::prelude::*;

use super::factory::RomeBuildingsFactoryResource;
use crate::{
    base::{buildings::components::BuildingGettingPlaced, factions::RomeBuildingType},
    game_state::GameState,
    states::UserActionState,
    terrain::TerrainResource,
};

pub struct RomeBuildingsUIPlugin;

impl Plugin for RomeBuildingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UserActionState>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (move_building_getting_placed.run_if(in_state(GameState::Playing)),),
            );
    }
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
