mod axis;
mod camera;

use axis::AxisPlugin;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        // User defined plugins
        .add_plugins(CameraPlugin)
        .add_plugins(AxisPlugin)
        .run();
}
