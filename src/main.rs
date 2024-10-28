use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};

mod prelude {
    pub use bevy::prelude::*;
    pub use std::f32::consts::PI;

    pub const EARTH_RADIUS: f32 = 6371.0; // 地球半径
}
use prelude::*;

use camera::OrbitCameraPlugin;
use satellite::SatellitePlugin;

mod camera;
mod satellite;

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

    // // 创建卫星
    // let satellite_material = materials.add(StandardMaterial {
    //     base_color: Color::srgb(0.8, 0.8, 0.8),
    //     unlit: true,
    //     ..Default::default()
    // });
}

// fn move_satellites(time: Res<Time>, mut query: Query<(&mut Satellite, &mut Transform)>) {
//     for (mut satellite, mut transform) in query.iter_mut() {
//         satellite.current_time += time.delta_seconds();
//         let mean_anomaly = (2.0 * PI / satellite.orbit_period) * satellite.current_time;
//         // 假设一个小的近似处理，用平均近点角近似真实近点角（稍微简化了开普勒方程）
//         let true_anomaly = mean_anomaly;
//         // 计算卫星在轨道平面内的位置
//         let radius = satellite.semi_major_axis * (1.0 - satellite.eccentricity.powi(2))
//             / (1.0 + satellite.eccentricity * true_anomaly.cos());
//         let x = radius * true_anomaly.cos();
//         let y = radius * true_anomaly.sin();
//         // 应用轨道倾角和近地点角距
//         let rotated_position = Quat::from_rotation_z(satellite.argument_of_periapsis)
//             * Quat::from_rotation_x(satellite.inclination)
//             * Vec3::new(x, y, 0.0);
//         transform.translation = rotated_position;
//     }
// }

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn draw_axes(mut gizmos: Gizmos) {
    gizmos.axes(Transform::default(), 1.5 * EARTH_RADIUS);
}
