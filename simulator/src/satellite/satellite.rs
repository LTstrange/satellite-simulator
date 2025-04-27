use super::*;

#[derive(Component)]
#[require(Transform)]
pub struct Satellite {
    pub mean_anomaly: f32, // 平近点角(rad)
}

#[derive(Component, Debug)]
#[relationship(relationship_target= FollowedBy)]
pub struct Following(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship= Following)]
pub struct FollowedBy(Vec<Entity>);

pub fn update_mean_anomaly(
    config: Res<Config>,
    orbits: Query<(&Orbit, &FollowedBy)>,
    mut satellites: Query<&mut Satellite>,
    time: Res<Time<Fixed>>,
) -> Result {
    for (orbit, sates) in orbits {
        let mean_motion = orbit.mean_motion;
        for sate in &sates.0 {
            let mut sate = satellites.get_mut(*sate)?;
            sate.mean_anomaly += mean_motion * time.delta_secs() * config.Simulation.time_speed;
            sate.mean_anomaly %= 2. * PI;
        }
    }

    Ok(())
}

pub fn update_satellite_position(
    orbits: Query<(&Orbit, &FollowedBy)>,
    mut satellites: Query<(&mut Transform, &Satellite)>,
) -> Result {
    for (orbit, sates) in orbits {
        for sate in &sates.0 {
            let (mut transform, sate) = satellites.get_mut(*sate)?;
            transform.translation = get_position_from_orbital_elements(
                &OrbitalElements::from_orbit_n_sate(orbit, sate),
            );
        }
    }

    Ok(())
}

pub fn get_position_from_orbital_elements(orbital: &OrbitalElements) -> Vec3 {
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
