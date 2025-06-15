use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RapierPickable};

use crate::{
    config::terrain::TerrainConfig,
    game_states::GameState,
    units::{MoveTo, Selected, UnitSelector, utils::remove_selection},
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Terrain>()
            .add_systems(OnEnter(GameState::Playing), setup);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Terrain;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_config_res: Res<TerrainConfig>,
) {
    println!("Spawing terrain");
    commands
        .spawn((
            Name::new("Terrain"),
            Terrain,
            StateScoped(GameState::Playing),
            Mesh3d(meshes.add(Plane3d::new(
                Vec3::Y,
                Vec2::new(terrain_config_res.x, terrain_config_res.y),
            ))),
            MeshMaterial3d(materials.add(Color::srgb_u8(111, 78, 55))),
            Transform::from_translation(Vec3::ZERO),
            Collider::cuboid(terrain_config_res.x, 0.01, terrain_config_res.y),
            RapierPickable,
        ))
        .observe(on_click);
}

fn on_click(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    selected_units: Query<Entity, (With<Selected>, Without<UnitSelector>)>,
    unit_selectors_selected: Query<Entity, (With<UnitSelector>, With<Selected>)>,
) {
    let hit = click.hit.position.unwrap();

    match click.button {
        PointerButton::Primary => {
            remove_selection(&mut commands, selected_units, unit_selectors_selected);
        }
        PointerButton::Secondary => {
            for unit in selected_units {
                commands.entity(unit).insert(MoveTo { target: hit.xz() });
            }
        }
        PointerButton::Middle => (),
    }
}
