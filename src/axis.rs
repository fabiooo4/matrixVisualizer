use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AxisPlugin;

impl Plugin for AxisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_axis);
    }
}

fn spawn_axis(mut gizmos: Gizmos) {
    let axis_color = Color::WHITE;

    // let basis = [Vec2::new(1., 0.), Vec2::new(0., 1.)];
    let basis = [Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(0., 0., 1.)];
    let matrix = Mat3::from_cols(basis[0], basis[1], basis[2]);

    // Cycle through the basis vectors and draw them
    for vector in matrix.to_cols_array_2d().iter() {
        gizmos.primitive_3d(
            Line3d {
                direction: Direction3d::new(Vec3::new(vector[0], vector[1], vector[2])).expect("Invalid direction"),
            },
            Vec3::ZERO,
            Quat::IDENTITY,
            axis_color,
        );
    }
}
