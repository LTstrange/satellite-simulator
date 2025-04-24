use serde::{Deserialize, Serialize};

const STANDARD_GRAVITATIONAL_PARAMETER: f32 = 3.986004418e5;

/// Add a Satellite
/// todo: replace by  AddOrbit and AddSatellite
#[derive(Debug, Serialize, Deserialize)]
pub struct AddSatellite {
    apogee: f32,
    perigee: f32,
    inclination: f32,
    longitude_of_ascending_node: f32,
    argument_of_periapsis: f32,
    mean_anomaly: f32,
}

impl AddSatellite {
    pub fn as_slice(&self) -> [f32; 6] {
        // e = (apogee - perigee) / (apogee + perigee)
        // mean_motion = (standard_gravitational_parameter /
        //                ((apogee + perigee)/2) ** 3) ** 0.5
        let e = (self.apogee - self.perigee) / (self.apogee + self.perigee);
        let mean_motion = (STANDARD_GRAVITATIONAL_PARAMETER
            / ((self.apogee + self.perigee) / 2.).powf(3.))
        .sqrt();
        [
            mean_motion,
            e,
            self.inclination,
            self.longitude_of_ascending_node,
            self.argument_of_periapsis,
            self.mean_anomaly,
        ]
    }
}
