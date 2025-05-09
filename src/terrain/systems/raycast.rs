use bevy::prelude::*;

use bevy_rapier3d::{pipeline::QueryFilter, plugin::ReadRapierContext};

use crate::terrain::{Terrain, TerrainResource};

pub fn raycast_cursor_to_terrain(
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
