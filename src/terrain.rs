use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RapierPickable};

use crate::{
    config::terrain::TerrainConfig,
    game_states::GameState,
    units::{MoveTo, Selected, UnitSelector},
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
    selected_units_query: Query<Entity, With<Selected>>,
    units_selector_query: Query<Entity, With<UnitSelector>>,
) {
    let hit = click.hit.position.unwrap();

    match click.button {
        PointerButton::Primary => {
            for entity in units_selector_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in selected_units_query.iter() {
                commands.entity(entity).remove::<Selected>();
            }
        }
        PointerButton::Secondary => {
            if let Ok(unit) = selected_units_query.single() {
                commands.entity(unit).insert(MoveTo { target: hit.xz() });
            }
        }
        PointerButton::Middle => (),
    }
}
