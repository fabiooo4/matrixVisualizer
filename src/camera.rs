use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(Debug, Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..default()
        }.into(),
        ..Default::default()
    },
        PanOrbitCamera {
            button_pan: MouseButton::Left,
            button_orbit: MouseButton::Right,
            ..default()
        }
    ));
}
