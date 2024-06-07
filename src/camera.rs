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
        transform: Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(6.0),
            ..default()
        }.into(),
        ..Default::default()
    },
        PanOrbitCamera::default(),
    ));
}
