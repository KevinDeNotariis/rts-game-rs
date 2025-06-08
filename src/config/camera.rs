use bevy::prelude::*;
use std::{f32::consts::PI, ops::Range};

pub struct CameraConfigPlugin;

impl Plugin for CameraConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraConfig>()
            .insert_resource(CameraConfig {
                orthographic_viewport_height: 5.,
                orthographic_zoom_range: 0.05..10.0,
                orthographic_zoom_speed: 0.001,
                perspective_zoom_range: (PI / 5.)..(PI - 0.2),
                perspective_zoom_speed: 0.05,
                movement_speed: 10.0,
            });
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct CameraConfig {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    /// in which one unit in world space corresponds to one pixel.
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
    /// Clamp perspective camera's field of view to this range
    pub perspective_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the perspective camera
    pub perspective_zoom_speed: f32,
    /// Speed with which the camera is going to be moved horizontally and vertically
    pub movement_speed: f32,
}
