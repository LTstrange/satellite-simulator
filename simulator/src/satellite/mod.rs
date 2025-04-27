use crate::prelude::*;
use chrono::Utc;

mod communication;
mod orbit;
mod satellite;

use communication::*;
use orbit::*;
use satellite::*;

const FACTOR: f32 = 73.594_6; // u^(1/3)

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommunicationPlugin);

        app.init_resource::<SatelliteSpawner>();

        app.add_event::<SpawnSatellites>();

        app.add_systems(Startup, setup)
            .add_systems(Update, (receive_spawn_event, spawn_satellites).chain())
            .add_systems(
                FixedUpdate,
                (update_mean_anomaly, update_satellite_position).chain(),
            );
    }
}

#[derive(Debug, Clone)]
pub struct OrbitalElements {
    mean_motion: f32,                 // 平均运动(rad/s)
    eccentricity: f32,                // 离心率
    inclination: f32,                 // 轨道倾角(rad)
    longitude_of_ascending_node: f32, // 升交点赤经(rad)
    argument_of_periapsis: f32,       // 近地点角距(rad)
    mean_anomaly: f32,                // 平近点角(rad)
}

impl OrbitalElements {
    fn new(value: &RawSatelliteData) -> Self {
        Self {
            mean_motion: value.MEAN_MOTION * 2. * PI / 86400.0, // rev/day to rad/s
            eccentricity: value.ECCENTRICITY,
            inclination: value.INCLINATION * PI / 180.0, // degrees to rad
            argument_of_periapsis: value.ARG_OF_PERICENTER * PI / 180.0, // degrees to rad
            longitude_of_ascending_node: value.RA_OF_ASC_NODE * PI / 180.0, // degrees to rad
            mean_anomaly: value.MEAN_ANOMALY * PI / 180.0, // degrees to rad
        }
    }

    pub fn from_orbit_n_sate(orbit: &Orbit, sate: &Satellite) -> Self {
        Self {
            mean_motion: orbit.mean_motion,
            eccentricity: orbit.eccentricity,
            inclination: orbit.inclination,
            longitude_of_ascending_node: orbit.longitude_of_ascending_node,
            argument_of_periapsis: orbit.argument_of_periapsis,
            mean_anomaly: sate.mean_anomaly,
        }
    }

    pub fn from_slice(data: &[f32; 6]) -> Result<Self, String> {
        let sate = Self {
            mean_motion: data[0],
            eccentricity: data[1],
            inclination: data[2],
            longitude_of_ascending_node: data[3],
            argument_of_periapsis: data[4],
            mean_anomaly: data[5],
        };
        if sate.eccentricity < 0.0 || sate.eccentricity >= 1.0 {
            return Err("Invalid eccentricity".to_string());
        }
        Ok(sate)
    }
}

#[derive(Resource, Default)]
struct SatelliteSpawner {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    unspawned_sats: Vec<(String, OrbitalElements)>,
}

#[derive(Event)]
pub struct SpawnSatellites {
    pub satellites: Vec<(String, OrbitalElements)>,
}

/// Read satellite data and Setup Satellite Spawner.
fn setup(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    config: Res<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result {
    let satellite_mesh = meshes.add(Sphere::new(20.).mesh().ico(1).unwrap());
    let satellite_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        unlit: true,
        ..default()
    });

    let data = if let Some(dataset) = &config.Dataset {
        let raw_satellites_datas = dataset.read_from_file()?;

        let current_time = Utc::now();

        raw_satellites_datas
            .into_iter()
            .map(|satellite_data| {
                let observe_time = parse_time_from_str(&satellite_data.EPOCH);
                let duration = current_time - observe_time.unwrap();
                let mut satellite = OrbitalElements::new(&satellite_data);
                satellite.mean_anomaly +=
                    (duration.num_seconds() as f32 * satellite.mean_motion) % (2. * PI);
                (satellite_data.OBJECT_ID, satellite)
            })
            .collect()
    } else {
        vec![]
    };

    let mut gizmo = GizmoAsset::default();
    for (satellite_id, satellite) in &data {
        draw_orbit_gizmo(satellite, &mut gizmo);
        // info!("Spawn orbit: {:?}", satellite_id);
        commands.spawn(orbit(
            satellite_id.to_string(),
            satellite,
            satellite_mesh.clone(),
            satellite_material.clone(),
        ));
    }

    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        ..default()
    });

    commands.insert_resource(SatelliteSpawner {
        mesh: satellite_mesh,
        material: satellite_material,
        unspawned_sats: data,
    });

    Ok(())
}

fn receive_spawn_event(
    mut event: EventReader<SpawnSatellites>,
    mut satellite_spawner: ResMut<SatelliteSpawner>,
) {
    for SpawnSatellites { satellites } in event.read() {
        println!("Receive spawn event: {}", satellites.len());
        satellite_spawner.unspawned_sats.extend(satellites.clone());
    }
}

fn spawn_satellites(mut commands: Commands, mut satellite_spawner: ResMut<SatelliteSpawner>) {
    let mesh = satellite_spawner.mesh.clone();
    let material = satellite_spawner.material.clone();
    for (satellite_id, satellite) in satellite_spawner.unspawned_sats.drain(..) {
        assert_ne!(mesh.clone(), Handle::default());
        assert_ne!(material.clone(), Handle::default());

        commands.spawn(orbit(
            satellite_id.to_string(),
            &satellite,
            mesh.clone(),
            material.clone(),
        ));
    }
}
