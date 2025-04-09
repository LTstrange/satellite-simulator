use super::*;

pub fn update_mean_anomaly(
    config: Res<Config>,
    mut satellites: Query<&mut Satellite, With<Satellite>>,
    time: Res<Time<Fixed>>,
) {
    for mut element in &mut satellites {
        element.mean_anomaly +=
            element.mean_motion * time.delta_secs() * config.Simulation.time_speed;
        element.mean_anomaly %= 2. * PI;
    }
}

pub fn update_satellite_position(mut satellites: Query<(&mut Transform, &Satellite)>) {
    for (mut transform, orbital) in satellites.iter_mut() {
        transform.translation = get_position_from_orbital_elements(orbital);
    }
}

pub fn get_position_from_orbital_elements(orbital: &Satellite) -> Vec3 {
    let true_anomaly = anomaly_mean_to_true(orbital.mean_anomaly, orbital.eccentricity).unwrap();
    let n = orbital.mean_motion.powf(-2. / 3.);
    let semi_major_axis = FACTOR * n;
    // r = a(1- e^2) / (1 + e * cos(true_anomaly))
    let radius = semi_major_axis * (1.0 - orbital.eccentricity.powi(2))
        / (1. + orbital.eccentricity * true_anomaly.cos());
    let local_location = Vec3::new(
        radius * true_anomaly.cos(),
        radius * true_anomaly.sin(),
        0.0,
    ); // location on the orbital plane

    let rot = get_rotated_quat(
        orbital.inclination,
        orbital.longitude_of_ascending_node,
        orbital.argument_of_periapsis,
    );

    rot * local_location // apply rotation
}
