use pyo3::prelude::*;
use std::f32::consts::PI;

#[pyclass]
struct SatSimManager {
    host: String,
    port: u16,
}

#[pymethods]
impl SatSimManager {
    #[new]
    /// Create a new SatSimManager.
    /// And connect to the Satellite Simulator.
    fn new(host: String, port: u16) -> PyResult<Self> {
        println!("Connecting to {}:{}", host, port);
        Ok(Self { host, port })
    }

    ///Create a new orbit.
    /// Args:
    /// apogee: The apogee distance in kilometers.
    /// perigee : The perigee distance in kilometers.
    /// inclination: The inclination in radians.
    /// longitude_of_ascending_node: The longitude of ascending node in radians.
    /// argument_of_periapsis: The argument of periapsis in radians.
    ///
    fn create_orbit(
        &self,
        apogee: f32,
        perigee: f32,
        inclination: f32,
        longitude_of_ascending_node: f32,
        argument_of_periapsis: f32,
        mean_anomaly: f32,
    ) {
        println!(
            "Creating orbit: apogee={} km, perigee={} km, inclination={} * pi, longitude_of_ascending_node={} * pi, argument_of_periapsis={} * pi, mean_anomaly={} * pi",
            apogee, perigee, inclination / PI, longitude_of_ascending_node / PI, argument_of_periapsis / PI, mean_anomaly / PI
        );
    }
}

#[pymodule]
fn orbiter_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SatSimManager>()?;
    Ok(())
}
