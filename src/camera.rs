use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement_keyboard);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn movement_keyboard(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result {
    let mut camera = camera_query.single_mut()?;

    let forward = camera.forward();
    let forward_unit = Dir3::from_xyz(forward.x, 0.0, forward.z).unwrap();

    let left = camera.left();
    let left_unit = Dir3::from_xyz(left.x, 0.0, left.z).unwrap();

    let speed = 3.0;

    if key.pressed(KeyCode::ArrowDown) {
        camera.translation -= forward_unit * time.delta_secs() * speed;
    }
    if key.pressed(KeyCode::ArrowUp) {
        camera.translation += forward_unit * time.delta_secs() * speed;
    }
    if key.pressed(KeyCode::ArrowLeft) {
        camera.translation += left_unit * time.delta_secs() * speed;
    }
    if key.pressed(KeyCode::ArrowRight) {
        camera.translation -= left_unit * time.delta_secs() * speed;
    }

    Ok(())
}
