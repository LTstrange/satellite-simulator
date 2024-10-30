use crate::prelude::*;
use chrono::Utc;
use std::fs::File;

mod communication;
mod motion;
mod orbit;

use communication::*;
use motion::*;
use orbit::*;

const FACTOR: f32 = 73.594_6; // u^(1/3)

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommunicationPlugin);
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
struct OrbitalElements {
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

/// Read and Setup satellite data and add them to the scene.
fn setup(
    mut commands: Commands,
    config: Res<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let data = File::open(&config.satellite_json).unwrap();
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

// helper functions
fn get_rotated_quat(
    inclination: f32,
    longitude_of_ascending_node: f32,
    argument_of_periapsis: f32,
) -> Quat {
    let mut quat = Quat::IDENTITY;

    // rotation
    quat = Quat::from_rotation_x(-inclination) * quat; // rotate_x
    quat = Quat::from_rotation_z(longitude_of_ascending_node) * quat; // rotate_z
    quat *= Quat::from_rotation_z(-argument_of_periapsis); // rotate_local_z
    quat
}
