use std::fs::File;

use crate::prelude::*;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_ellipse_orbit_data).chain())
            .add_systems(Update, draw_ellipse_orbit);
    }
}

// marker type for satellite
#[derive(Component)]
pub struct Satellite;

#[derive(Component)]
pub struct OrbitalElements {
    mean_motion: f32,                 // 平均运动(rad/s)
    eccentricity: f32,                // 离心率
    inclination: f32,                 // 轨道倾角(rad)
    argument_of_periapsis: f32,       // 近地点角距(rad)
    longitude_of_ascending_node: f32, // 升交点赤经(rad)
    mean_anomaly: f32,                // 平近点角(rad)
}

impl From<SatelliteData> for OrbitalElements {
    fn from(value: SatelliteData) -> Self {
        Self {
            mean_motion: value.MEAN_MOTION * 2. * PI / 86400.0, // rev/day to rad/s
            eccentricity: value.ECCENTRICITY,
            inclination: value.INCLINATION,
            argument_of_periapsis: value.ARG_OF_PERICENTER,
            longitude_of_ascending_node: value.RA_OF_ASC_NODE,
            mean_anomaly: value.MEAN_ANOMALY,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let data = File::open("./starlink.json").unwrap();
    let satellites: Vec<SatelliteData> = serde_json::from_reader(data).unwrap();

    let satellite_mesh = meshes.add(Sphere::new(20.).mesh().ico(2).unwrap());
    let satellite_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 1.0, 1.0),
        unlit: true,
        ..default()
    });
    for satellite in satellites {
        let orbital = OrbitalElements::from(satellite);

        let true_anomaly =
            anomaly_mean_to_true(orbital.mean_anomaly, orbital.eccentricity).unwrap();
        let n = orbital.mean_motion.powf(-2. / 3.);
        let semi_major_axis = FACTOR * n;
        // r = a(1- e^2) / (1 + e * cos(true_anomaly))
        let radius = (1.0 - orbital.eccentricity.powi(2)) * semi_major_axis
            / (1. + orbital.eccentricity * true_anomaly.cos());
        println!("radius: {}", radius);
        let location = Vec3::new(
            radius * true_anomaly.cos(),
            radius * true_anomaly.sin(),
            0.0,
        );

        let mut transform = Transform::from_translation(location);
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-orbital.inclination));
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(orbital.longitude_of_ascending_node),
        );
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(*transform.forward(), -orbital.argument_of_periapsis),
        );

        commands.spawn((
            Satellite,
            orbital,
            PbrBundle {
                transform,
                mesh: satellite_mesh.clone(),
                material: satellite_material.clone(),
                ..default()
            },
        ));
    }
}

#[derive(Component)]
struct EllipseOrbitData {
    location: Vec3,
    rotation: Quat,
    half_size: Vec2,
}

const FACTOR: f32 = 73.59459595; // u^(1/3)
fn setup_ellipse_orbit_data(mut commands: Commands, orbits: Query<&OrbitalElements>) {
    for element in &orbits {
        // half size of the ellipse

        let n = element.mean_motion.powf(-2. / 3.);
        // a = u^(1/3) * ( n ) ^ (-2/3)
        let semi_major_axis = FACTOR * n;
        // b = a * sqrt(1 - e^2)
        let semi_minor_axis = semi_major_axis * (1.0 - element.eccentricity.powi(2)).sqrt();
        let half_size = Vec2::new(semi_minor_axis, semi_major_axis);

        let mut transform = Transform::default();

        // rotation

        transform.rotate_local_y(-element.inclination);
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_z(element.longitude_of_ascending_node),
        );
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(*transform.forward(), -element.argument_of_periapsis),
        );

        // position
        // e = c / a; c = e * a
        let semi_focal_distance = semi_major_axis * element.eccentricity;
        transform.translation += semi_focal_distance * transform.local_y();

        commands.spawn(EllipseOrbitData {
            location: transform.translation,
            rotation: transform.rotation,
            half_size,
        });
    }
}

// Gizmos
fn draw_ellipse_orbit(mut gizmos: Gizmos, query: Query<&EllipseOrbitData>) {
    for ellpise in &query {
        gizmos.ellipse(
            ellpise.location,
            ellpise.rotation,
            ellpise.half_size,
            Color::srgba(1., 1., 1., 0.01),
        );
    }
}
