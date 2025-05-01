use bevy::{input::mouse, prelude::*};
use bevy_rapier3d::{
    pipeline::QueryFilter,
    plugin::ReadRapierContext,
    prelude::{Collider, RapierPickable},
    render::ColliderDebugColor,
};

use crate::{game_state::GameState, terrain::Terrain};

pub struct RomeBuildingsUIPlugin;

impl Plugin for RomeBuildingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, spawn_building.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
struct RomeBuildingsUIData {
    spawner_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RomeBuildingsUIFrame;

fn setup(mut commands: Commands) {
    let entity = commands
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
                    Text::new("Rome Spawn Building"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ]
            )],
        ))
        .id();

    commands.insert_resource(RomeBuildingsUIData {
        spawner_entity: entity,
    });
}

fn spawn_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RomeBuildingsUIFrame>)>,
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands
                    .spawn((
                        Name::new("RomeUnitsSpawn"),
                        Transform::from_xyz(0., 0., 0.),
                        Mesh3d(meshes.add(Cuboid::from_length(2.))),
                        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.7, 0.3))),
                        Collider::cuboid(1., 1., 1.),
                        ColliderDebugColor(Color::srgb(0., 0., 1.).into()),
                        RapierPickable,
                    ))
                    .observe(clicked_building)
                    .observe(on_drag_move);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn clicked_building(_click: Trigger<Pointer<Click>>) {
    println!("clicked");
}

// fn on_drag(
//     drag: Trigger<Pointer<Drag>>,
//     // mut ray_cast: MeshRayCast,
//     mut transform_q: Query<&mut Transform>,
//     // terrain_q: Query<&Transform, With<Terrain>>,
// ) {
//     // let terrain = terrain_q.single();

//     if let Ok(mut transform) = transform_q.get_mut(drag.target()) {
//         // let ray = Ray3d::new()
//         transform.rotate_y(drag.delta.x * 0.02);
//         transform.rotate_x(drag.delta.y * 0.02);
//     }
// }

fn on_drag_move(
    drag: Trigger<Pointer<Drag>>,
    mut transform_q: Query<&mut Transform>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    terrain_q: Query<Entity, With<Terrain>>,
    window: Query<&Window>,
    rapier_context: ReadRapierContext,
) -> Result {
    let mut transform = transform_q.get_mut(drag.target())?;

    // Get camera info
    let (camera, camera_transform) = camera_q.single()?;

    // Get terrain info
    let terrain = terrain_q.single()?;

    // Mouse position
    let mouse_position = window.single()?.cursor_position().unwrap(); // TODO: Dangerous

    // Convert screen position to ray
    let ray = camera.viewport_to_world(camera_transform, mouse_position)?; // Cast ray to find intersection with terrain
    let (_, intersection) = rapier_context
        .single()
        .unwrap()
        .cast_ray(
            ray.origin,
            ray.direction.into(),
            f32::MAX,
            true,
            QueryFilter::default().predicate(&|entity| entity == terrain),
        )
        .unwrap(); //TODO: Dangerous

    // Calculate the world position of the intersection
    let hit_position = ray.origin + ray.direction * intersection;
    println!("{:#?}", hit_position);

    // Update cube position to follow cursor on terrain
    // You might want to keep the original y-height of the object
    // or offset it slightly above the terrain
    let current_height = transform.translation.y;
    transform.translation = Vec3::new(
        hit_position.x,
        current_height, // or hit_position.y + offset
        hit_position.z,
    );

    Ok(())
}
