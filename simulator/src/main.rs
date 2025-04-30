mod prelude {
    pub use bevy::prelude::*;
    pub use serde::Deserialize;

    pub use super::config::*;
    pub use super::utils::*;

    pub use std::f32::consts::PI;
    pub const EARTH_RADIUS: f32 = 6371.0; // 地球半径
}

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use prelude::*;

use camera::OrbitCameraPlugin;
use satellite::SatellitePlugin;
use ui::UserInterfacePlugin;

mod camera;
mod config;
// mod io;
mod satellite;
mod ui;
mod utils;

fn main() -> Result {
    let mut config_path = std::env::current_dir()?;
    config_path.push("config.toml");
    println!("Loading Config file... : {:?}", config_path);
    let config = config::Config::load(&config_path)?;

    let _port = config.network.port;

    App::new()
        .insert_resource(config)
        .add_plugins(DefaultPlugins)
        .add_plugins((
            OrbitCameraPlugin,
            SatellitePlugin,
            // io::IOPlugin::new(port),
            UserInterfacePlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
    Ok(())
}

fn setup(
    mut commands: Commands,
    mut _images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    // 创建一个蓝色材质表示地球
    let earth_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.3, 0.7),
        unlit: true, // 无光照
        ..Default::default()
    });

    // 创建一个球体并设置其位置在原点
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(EARTH_RADIUS).mesh().uv(32, 18))),
        MeshMaterial3d(earth_material),
    ));

    // 创建坐标轴
    let mut gizmo = GizmoAsset::default();
    gizmo.axes(Transform::default(), 1.5 * EARTH_RADIUS);
    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        ..default()
    });
}
