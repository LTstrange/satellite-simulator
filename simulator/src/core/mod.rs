use crate::prelude::*;
use chrono::{DateTime, Utc};

mod communication;
mod manager;
mod orbit;
mod satellite;

use communication::*;
use manager::*;
use orbit::*;
use satellite::*;

pub use manager::{AttachSatellites, SpawnOrbits, SpawnSatellites};
pub use orbit::{Orbit, ToggleOrbitGizmos};
pub use satellite::Satellite;

const FACTOR: f32 = 73.594_6; // u^(1/3)

/// A Core Plugin for the simulator.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CommunicationPlugin,
            ManagerPlugin,
            SatellitePlugin,
            OrbitPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

/// A struct representing the orbital elements of a satellite.
/// Which is the cohesive data structure for the satellite and orbit at once.
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
    /// Create a new `OrbitalElements` instance from raw satellite data.
    /// By convert the units of the data. And update the mean anomaly by the current time.
    fn from_raw_sate_data(raw_data: RawSatelliteData, current_time: DateTime<Utc>) -> Result<Self> {
        let observe_time = parse_time_from_str(&raw_data.epoch)?;

        // convert units
        let mut orbit_elements = Self {
            mean_motion: raw_data.mean_motion * 2. * PI / 86400.0, // rev/day to rad/s
            eccentricity: raw_data.eccentricity,
            inclination: raw_data.inclination * PI / 180.0, // degrees to rad
            argument_of_periapsis: raw_data.arg_of_pericenter * PI / 180.0, // degrees to rad
            longitude_of_ascending_node: raw_data.ra_of_asc_node * PI / 180.0, // degrees to rad
            mean_anomaly: raw_data.mean_anomaly * PI / 180.0, // degrees to rad
        };

        // update mean anomaly by current time
        let duration = current_time - observe_time;
        orbit_elements.mean_anomaly +=
            (duration.num_seconds() as f32 * orbit_elements.mean_motion) % (2. * PI);
        Ok(orbit_elements)
    }

    fn sep_out_mean_anomaly(self) -> (Orbit, f32) {
        (
            Orbit {
                mean_motion: self.mean_motion,
                eccentricity: self.eccentricity,
                inclination: self.inclination,
                longitude_of_ascending_node: self.longitude_of_ascending_node,
                argument_of_periapsis: self.argument_of_periapsis,
            },
            self.mean_anomaly,
        )
    }

    #[allow(unused)]
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

/// Read satellite data and Setup Satellite Manager.
fn setup(
    config: Res<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut manager: ResMut<SatelliteManager>,
) -> Result {
    let satellite_mesh = meshes.add(Sphere::new(20.).mesh().ico(1).unwrap());
    let satellite_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        unlit: true,
        ..default()
    });
    manager.init(satellite_mesh, satellite_material);

    // read satellite data
    let data = if let Some(dataset) = &config.dataset {
        let current_time = Utc::now();

        dataset
            .read_from_file()?
            .into_iter()
            .map(|satellite_data| {
                let object_id = satellite_data.object_id.clone();
                let satellite = OrbitalElements::from_raw_sate_data(satellite_data, current_time)?;
                Ok((object_id, satellite))
            })
            .collect::<Result<Vec<(String, OrbitalElements)>>>()?
    } else {
        vec![]
    };

    manager.add_satellites(data);

    Ok(())
}
