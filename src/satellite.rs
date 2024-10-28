use crate::prelude::*;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, draw_ellipse_orbit);
    }
}

// marker type for satellite
#[derive(Component)]
pub struct Satellite;

#[derive(Component)]
pub struct OrbitalElements {
    semi_major_axis: f32,             // 半长轴
    eccentricity: f32,                // 离心率
    inclination: f32,                 // 轨道倾角
    argument_of_periapsis: f32,       // 近地点角距
    longitude_of_ascending_node: f32, // 升交点赤经
    true_anomaly: f32,                // 真近点角
}

impl OrbitalElements {
    pub fn new(
        semi_major_axis: f32,
        eccentricity: f32,
        inclination: f32,
        argument_of_periapsis: f32,
        longitude_of_ascending_node: f32,
        true_anomaly: f32,
    ) -> Self {
        Self {
            semi_major_axis,
            eccentricity,
            inclination,
            argument_of_periapsis,
            longitude_of_ascending_node,
            true_anomaly,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Satellite,
        OrbitalElements {
            semi_major_axis: EARTH_RADIUS + 6000.,
            eccentricity: 0.5,
            inclination: PI / 12.,
            argument_of_periapsis: PI / 4.,
            longitude_of_ascending_node: PI / 6.,
            true_anomaly: 0.,
        },
    ));
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

// Gizmos
fn draw_ellipse_orbit(mut gizmos: Gizmos, query: Query<&OrbitalElements>) {
    for orbit in &query {
        // half size of the ellipse
        // b = a * sqrt(1 - e^2)
        let semi_minor_axis = orbit.semi_major_axis * (1.0 - orbit.eccentricity.powi(2)).sqrt();
        let half_size = Vec2::new(semi_minor_axis, orbit.semi_major_axis);

        let mut transform = Transform::default();

        // rotation
        transform.rotate_local_y(-orbit.inclination);
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(orbit.longitude_of_ascending_node),
        );
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(*transform.forward(), -orbit.argument_of_periapsis),
        );

        // position
        // e = c / a; c = e * a
        let semi_focal_distance = orbit.semi_major_axis * orbit.eccentricity;
        transform.translation += semi_focal_distance * transform.local_y();

        gizmos.ellipse(
            transform.translation,
            transform.rotation,
            half_size,
            Color::WHITE,
        );
    }
}
