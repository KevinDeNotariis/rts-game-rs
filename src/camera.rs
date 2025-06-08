use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, render::camera::ScalingMode};
use bevy_rapier3d::prelude::RapierPickable;

use crate::config::camera::CameraConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (zoom, movement_keyboard));
    }
}

fn setup(mut commands: Commands, camera_settings: Res<CameraConfig>) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // We can set the scaling mode to FixedVertical to keep the viewport height constant as its aspect ratio changes.
            // The viewport height is the height of the camera's view in world units when the scale is 1.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.orthographic_viewport_height,
            }, // This is the default value for scale for orthographic projections.
            // To zoom in and out, change this value, rather than `ScalingMode` or the camera's position.
            scale: 1.,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        RapierPickable,
    ));
}

fn movement_keyboard(
    camera_query: Single<(&mut Transform, &mut Projection), With<Camera3d>>,
    camera_config: Res<CameraConfig>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result {
    let (mut camera, mut projection) = camera_query.into_inner();

    let forward = camera.forward();
    let forward_unit = Dir3::from_xyz(forward.x, 0.0, forward.z).unwrap();

    let left = camera.left();
    let left_unit = Dir3::from_xyz(left.x, 0.0, left.z).unwrap();

    let mut speed = camera_config.movement_speed;

    // If we are zoomed-in, the camera speed movement should be adjusted accordingly
    match projection.as_mut() {
        Projection::Orthographic(orthographic) => {
            speed = speed * orthographic.scale;
        }
        _ => {}
    }

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

fn zoom(
    camera: Single<&mut Projection, With<Camera3d>>,
    camera_settings: Res<CameraConfig>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.orthographic_zoom_range.start,
                camera_settings.orthographic_zoom_range.end,
            );
        }
        Projection::Perspective(ref mut perspective) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.perspective_zoom_speed;

            // Adjust the field of view, but keep it within our stated range.
            perspective.fov = (perspective.fov + delta_zoom).clamp(
                camera_settings.perspective_zoom_range.start,
                camera_settings.perspective_zoom_range.end,
            );
        }
        _ => (),
    }
}
