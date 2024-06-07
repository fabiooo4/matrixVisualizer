mod axis;
mod camera;

use axis::AxisPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AxisPlugin)
        .run();
}
