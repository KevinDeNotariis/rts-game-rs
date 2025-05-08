use bevy::prelude::*;
use bevy_rapier3d::{
    pipeline::QueryFilter, plugin::ReadRapierContext, prelude::Collider, render::ColliderDebugColor,
};

use crate::game_state::GameState;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TerrainResource {
    pub cursor_projection: Vec2,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Terrain;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainResource {
            cursor_projection: Vec2::ZERO,
        })
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            raycast_cursor_to_terrain.run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_size = 5.0;

    commands.spawn((
        Name::new("Terrain"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(terrain_size.clone())))),
        MeshMaterial3d(materials.add(Color::srgb_u8(111, 78, 55))),
        Transform::from_xyz(0.0, -1.0, 0.0),
        Collider::cuboid(terrain_size.clone(), 0.01, terrain_size.clone()),
        ColliderDebugColor(Color::srgb(1.0, 0.0, 0.0).into()),
        Terrain,
    ));
}

fn raycast_cursor_to_terrain(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    terrain_q: Query<Entity, With<Terrain>>,
    window: Query<&Window>,
    rapier_context: ReadRapierContext,
    mut terrain_resource: ResMut<TerrainResource>,
) -> Result {
    // Get camera info
    let (camera, camera_transform) = camera_q.single()?;

    // Get terrain info
    let terrain = terrain_q.single()?;

    // Mouse position
    let mouse_position = window.single()?.cursor_position().unwrap();

    // Convert screen position to ray
    let ray = camera.viewport_to_world(camera_transform, mouse_position)?;

    // Cast ray to find intersection with terrain
    if let Some((_, intersection)) = rapier_context.single()?.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::default().predicate(&|entity| entity == terrain),
    ) {
        // Calculate the world position of the intersection
        let hit_position = ray.origin + ray.direction * intersection;

        terrain_resource.cursor_projection = Vec2::new(hit_position.x, hit_position.z);
    }

    Ok(())
}
