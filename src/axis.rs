use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AxisPlugin;

impl Plugin for AxisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_basis);
        app.add_systems(Update, (spawn_axis,));
    }
}

#[derive(Debug, Component)]
struct Basis3d {
    basis: [Vec3; 3],
}

fn setup_basis(mut commands: Commands) {
    let matrix = Mat3::from_cols(
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 1., 0.),
        Vec3::new(0., 0., 1.),
    );

    commands.spawn((Basis3d {
        basis: matrix
            .to_cols_array_2d()
            .iter()
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect::<Vec<Vec3>>()
            .try_into()
            .expect("Invalid basis"),
    },));
}

fn spawn_axis(mut commands: Commands, mut gizmos: Gizmos, query: Query<&Basis3d>) {
    let axis_color = [
        Color::rgb(255. / 255., 17. / 255., 85. / 255.),
        Color::rgb(17. / 255., 252. / 255., 168. / 255.),
        Color::rgb(51. / 255., 187. / 255., 255. / 255.),
    ];

    // Cycle through the basis vectors and draw them
    commands.spawn(
        for (i, vector) in query
            .iter()
            .next()
            .expect("No basis found")
            .basis
            .iter()
            .enumerate()
        {
            gizmos.primitive_3d(
                Line3d {
                    direction: Direction3d::new(*vector).expect("Invalid direction"),
                },
                Vec3::ZERO,
                Quat::IDENTITY,
                axis_color[i],
            );
        },
    ).with_children(|parent| {
            // // Draw 100 lines along x axis
            // for i in -100..100 {
            //     let color = Color::rgba(0.5, 0.5, 0.5, 0.1);
            //     let offset = query.iter().next().unwrap().basis[0] * i as f32;
            //     parent.spawn(for vector in query
            //         .iter()
            //         .next()
            //         .expect("No basis found")
            //         .basis
            //     {
            //         gizmos.primitive_3d(
            //             Segment3d {
            //                 direction: Direction3d::new(vector).expect("Invalid direction"),
            //                 half_length: 50.0,
            //             },
            //             offset,
            //             Quat::IDENTITY,
            //             color,
            //         );
            //     });
            // }
        });

}

fn update_y_basis(mut query: Query<&mut Basis3d>) {
    // Rotate the basis vectors around the z-axis by 0.01 radians per second
    let rotation = Quat::from_rotation_z(0.1 * 1.0 / 60.0);
    for vector in query
        .iter_mut()
        .next()
        .expect("No basis found")
        .basis
        .iter_mut()
    {
        *vector = rotation.mul_vec3(*vector);
    }
}
