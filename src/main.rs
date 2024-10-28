mod prelude {
    pub use bevy::prelude::*;

    pub use std::f32::consts::PI;
    pub const EARTH_RADIUS: f32 = 6371.0; // 地球半径
}
use prelude::*;

use camera::OrbitCameraPlugin;
use satellite::SatellitePlugin;
use utils::*;

mod camera;
mod satellite;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((OrbitCameraPlugin, SatellitePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_axes,))
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 创建一个蓝色材质表示地球
    // let earth_material = materials.add(StandardMaterial {
    //     base_color: Color::srgb(0.0, 0.3, 0.7),
    //     unlit: true, // 无光照
    //     ..Default::default()
    // });
    let earth_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        unlit: true,
        ..default()
    });

    // 创建一个球体并设置其位置在原点
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(EARTH_RADIUS).mesh().uv(32, 18)),
        material: earth_material,
        ..Default::default()
    });
}

fn draw_axes(mut gizmos: Gizmos) {
    gizmos.axes(Transform::default(), 1.5 * EARTH_RADIUS);
}
