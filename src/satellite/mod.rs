use crate::prelude::*;
use chrono::Utc;
use std::fs::File;

mod communication;
mod motion;
mod orbit;

use communication::*;
use motion::*;
use orbit::*;

pub use communication::DisconnectAll;

const FACTOR: f32 = 73.594_6; // u^(1/3)

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommunicationPlugin);

        app.add_event::<SpawnSatellite>();

        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                draw_ellipse_orbit,
                (receive_spawn_event, spawn_satellites).chain(),
            ),
        );
        app.add_systems(
            FixedUpdate,
            (update_mean_anomaly, update_satellite_position).chain(),
        );
    }
}

#[derive(Component, Debug, Clone)]
#[require(Connections)]
pub struct Satellite {
    mean_motion: f32,                 // 平均运动(rad/s)
    eccentricity: f32,                // 离心率
    inclination: f32,                 // 轨道倾角(rad)
    longitude_of_ascending_node: f32, // 升交点赤经(rad)
    argument_of_periapsis: f32,       // 近地点角距(rad)
    mean_anomaly: f32,                // 平近点角(rad)
}

impl Satellite {
    pub fn new(value: &SatelliteData) -> Self {
        Self {
            mean_motion: value.MEAN_MOTION * 2. * PI / 86400.0, // rev/day to rad/s
            eccentricity: value.ECCENTRICITY,
            inclination: value.INCLINATION * PI / 180.0, // degrees to rad
            argument_of_periapsis: value.ARG_OF_PERICENTER * PI / 180.0, // degrees to rad
            longitude_of_ascending_node: value.RA_OF_ASC_NODE * PI / 180.0, // degrees to rad
            mean_anomaly: value.MEAN_ANOMALY * PI / 180.0, // degrees to rad
        }
    }
    pub fn from_slice(data: &[f32; 6]) -> Self {
        Self {
            mean_motion: data[0],
            eccentricity: data[1],
            inclination: data[2],
            longitude_of_ascending_node: data[3],
            argument_of_periapsis: data[4],
            mean_anomaly: data[5],
        }
    }
}

#[derive(Resource)]
struct SatelliteSpawner {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    data: Vec<(String, Satellite)>,
}

#[derive(Event)]
pub struct SpawnSatellite {
    pub id: String,
    pub data: Satellite,
}

/// Read and Setup satellite data and add them to the scene.
fn setup(
    mut commands: Commands,
    config: Res<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(data_file) = &config.Dataset {
        let data = File::open(data_file.constellation_file.clone()).unwrap();
        let satellites_data: Vec<SatelliteData> = serde_json::from_reader(data).unwrap();

        let satellite_mesh = meshes.add(Sphere::new(20.).mesh().ico(1).unwrap());
        let satellite_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            unlit: true,
            ..default()
        });

        let current_time = Utc::now();
        let mut data = Vec::new();

        data.extend(satellites_data.into_iter().map(|satellite_data| {
            let observe_time = parse_time_from_str(&satellite_data.EPOCH);
            let duration = current_time - observe_time.unwrap();
            let mut satellite = Satellite::new(&satellite_data);
            satellite.mean_anomaly +=
                (duration.num_seconds() as f32 * satellite.mean_motion) % (2. * PI);
            (satellite_data.OBJECT_ID, satellite)
        }));

        commands.insert_resource(SatelliteSpawner {
            mesh: satellite_mesh,
            material: satellite_material,
            data,
        });
    }
}

fn receive_spawn_event(
    mut event: EventReader<SpawnSatellite>,
    mut satellite_spawner: ResMut<SatelliteSpawner>,
) {
    for SpawnSatellite { id, data } in event.read() {
        satellite_spawner.data.push((id.clone(), data.clone()));
    }
}

fn spawn_satellites(mut commands: Commands, mut satellite_spawner: ResMut<SatelliteSpawner>) {
    let mesh = satellite_spawner.mesh.clone();
    let material = satellite_spawner.material.clone();
    for (satellite_id, satellite) in satellite_spawner.data.drain(..) {
        let pos = get_position_from_orbital_elements(&satellite);
        let orbit = get_ellipse_orbit_data(&satellite);
        commands.spawn((
            satellite,
            Name::new(satellite_id),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(pos),
            orbit,
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
