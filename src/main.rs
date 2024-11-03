mod prelude {
    pub use anyhow::Result;
    pub use bevy::prelude::*;
    pub use serde::Deserialize;

    pub use super::config::*;
    pub use super::satellite::*;
    pub use super::utils::*;

    pub use std::f32::consts::PI;
    pub const EARTH_RADIUS: f32 = 6371.0; // 地球半径
}

use prelude::*;

use camera::OrbitCameraPlugin;
use control::ControlPlugin;
use satellite::SatellitePlugin;

mod camera;
mod config;
mod control;
mod satellite;
mod utils;

fn main() -> Result<()> {
    let mut config_path = std::env::current_dir()?;
    config_path.push("config.toml");
    let config = config::Config::load(&config_path)?;
    App::new()
        .insert_resource(config)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((OrbitCameraPlugin, SatellitePlugin, ControlPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_axes,))
        .run();
    Ok(())
}

fn setup(
    mut commands: Commands,
    mut _images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 创建一个蓝色材质表示地球
    let earth_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.3, 0.7),
        unlit: true, // 无光照
        ..Default::default()
    });
    // let earth_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     unlit: true,
    //     ..default()
    // });

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
