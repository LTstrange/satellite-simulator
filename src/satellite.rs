use chrono::Utc;
use std::fs::File;

use crate::prelude::*;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_ellipse_orbit_data).chain())
            .add_systems(Update, draw_ellipse_orbit);
        app.add_systems(
            FixedUpdate,
            (update_mean_anomaly, update_satellite_position).chain(),
        );
    }
}

// marker type for satellite
#[derive(Component)]
pub struct Satellite;

#[derive(Component, Debug)]
pub struct OrbitalElements {
    mean_motion: f32,                 // 平均运动(rad/s)
    eccentricity: f32,                // 离心率
    inclination: f32,                 // 轨道倾角(rad)
    longitude_of_ascending_node: f32, // 升交点赤经(rad)
    argument_of_periapsis: f32,       // 近地点角距(rad)
    mean_anomaly: f32,                // 平近点角(rad)
}

impl From<SatelliteData> for OrbitalElements {
    fn from(value: SatelliteData) -> Self {
        Self {
            mean_motion: value.MEAN_MOTION * 2. * PI / 86400.0, // rev/day to rad/s
            eccentricity: value.ECCENTRICITY,
            inclination: value.INCLINATION * PI / 180.0, // degrees to rad
            argument_of_periapsis: value.ARG_OF_PERICENTER * PI / 180.0, // degrees to rad
            longitude_of_ascending_node: value.RA_OF_ASC_NODE * PI / 180.0, // degrees to rad
            mean_anomaly: value.MEAN_ANOMALY * PI / 180.0, // degrees to rad
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

    let satellite_mesh = meshes.add(Sphere::new(20.).mesh().ico(1).unwrap());
    let satellite_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        unlit: true,
        ..default()
    });

    let current_time = Utc::now();
    for satellite in satellites {
        let observe_time = parse_time_from_str(&satellite.EPOCH);

        let duration = current_time - observe_time.unwrap();

        let mut orbital = OrbitalElements::from(satellite);
        orbital.mean_anomaly += (duration.num_seconds() as f32 * orbital.mean_motion) % (2. * PI);

        let pos = get_position_from_orbital_elements(&orbital);

        commands.spawn((
            Satellite,
            orbital,
            PbrBundle {
                transform: Transform::from_translation(pos),
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
        transform.rotate_x(-element.inclination);
        transform.rotate_z(element.longitude_of_ascending_node);
        transform.rotate_local_z(-element.argument_of_periapsis);

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

fn update_mean_anomaly(
    mut satellites: Query<&mut OrbitalElements, With<Satellite>>,
    time: Res<Time<Fixed>>,
) {
    for mut element in &mut satellites {
        element.mean_anomaly += element.mean_motion * time.delta_seconds() * 100.;
        element.mean_anomaly %= 2. * PI;
    }
}

fn update_satellite_position(mut satellites: Query<(&mut Transform, &OrbitalElements)>) {
    for (mut transform, orbital) in satellites.iter_mut() {
        transform.translation = get_position_from_orbital_elements(orbital);
    }
}

pub fn get_position_from_orbital_elements(orbital: &OrbitalElements) -> Vec3 {
    let true_anomaly = anomaly_mean_to_true(orbital.mean_anomaly, orbital.eccentricity).unwrap();
    let n = orbital.mean_motion.powf(-2. / 3.);
    let semi_major_axis = FACTOR * n;
    // r = a(1- e^2) / (1 + e * cos(true_anomaly))
    let radius = semi_major_axis * (1.0 - orbital.eccentricity.powi(2))
        / (1. + orbital.eccentricity * true_anomaly.cos());
    let location = Vec3::new(
        radius * true_anomaly.cos(),
        radius * true_anomaly.sin(),
        0.0,
    );

    let mut transform = Transform::default();

    // rotation
    transform.rotate_y(-orbital.inclination);
    transform.rotate_z(orbital.longitude_of_ascending_node);
    transform.rotate_local_z(-orbital.argument_of_periapsis);

    // position
    transform.transform_point(location)
}
