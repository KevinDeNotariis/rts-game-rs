use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_rapier3d::prelude::{Collider, RapierPickable};

use crate::game_states::GameState;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct UnitSelector;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[component(on_add = selected_added)]
pub struct Selected;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MoveTo {
    pub target: Vec2,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform, Collider)]
pub struct Unit;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.1_f32;
    let half_length = 0.3_f32;

    commands
        .spawn((
            Name::new("unit"),
            Unit,
            Mesh3d(meshes.add(Capsule3d::new(radius, half_length))),
            Transform::from_translation(Vec3::new(0., half_length, 0.)),
            MeshMaterial3d(materials.add(Color::srgb_u8(50, 50, 200))),
            Collider::capsule_y(half_length / 2., radius),
            RapierPickable,
            Movement { speed: 1.0 },
        ))
        .observe(on_click);
}

fn on_click(click: Trigger<Pointer<Click>>, mut commands: Commands) {
    match click.button {
        PointerButton::Primary => commands.entity(click.target).insert(Selected),
        PointerButton::Secondary => todo!(),
        PointerButton::Middle => todo!(),
    };
}

fn selected_added(mut world: DeferredWorld, context: HookContext) {
    let mesh_handle = {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        meshes.add(Torus::new(0.15, 0.17))
    };

    let material_handle = {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        materials.add(Color::srgb_u8(0, 200, 0))
    };

    let mut commands = world.commands();

    commands.entity(context.entity).insert(children![(
        Name::new("Selector"),
        UnitSelector,
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0., -0.3, 0.),
    )]);
}

fn close_enough(v1: Vec2, v2: Vec2) -> bool {
    // We use distance_squared as square roots are generally computationally heavy and in this case
    // we do not need that amount of precision.
    v1.distance_squared(v2) < f32::EPSILON
}

fn movement(
    mut commands: Commands,
    mut units_to_move_query: Query<(&mut Transform, &Movement, &MoveTo, Entity), With<Movement>>,
    time: Res<Time>,
) {
    for (mut transform, movement, move_to, entity) in units_to_move_query.iter_mut() {
        let origin = transform.translation.xz();
        let destination = move_to.target;

        if close_enough(origin, destination) {
            commands.entity(entity).remove::<MoveTo>();
        }

        let delta = time.delta_secs() * movement.speed;
        let direction = (destination - origin).normalize();

        transform.translation += delta * Vec3::new(direction.x, 0., direction.y);
    }
}
