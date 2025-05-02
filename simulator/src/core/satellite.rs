use super::*;

pub struct SatellitePlugin;
impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_mean_anomaly, update_satellite_position).chain(),
        );
    }
}

#[derive(Component, Clone)]
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

pub fn create_satellite(
    satellite_id: String,
    orbit_entity: Entity,
    mean_anomaly: f32,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) -> impl Bundle {
    (
        Satellite { mean_anomaly },
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Name::new(satellite_id),
        Following(orbit_entity),
    )
}

fn update_mean_anomaly(
    config: Res<Config>,
    orbits: Query<(&Orbit, &FollowedBy)>,
    mut satellites: Query<&mut Satellite>,
    time: Res<Time<Fixed>>,
) -> Result {
    for (orbit, sates) in orbits {
        let mean_motion = orbit.mean_motion;
        for sate in &sates.0 {
            let mut sate = satellites.get_mut(*sate)?;
            sate.mean_anomaly += mean_motion * time.delta_secs() * config.simulation.time_speed;
            sate.mean_anomaly %= 2. * PI;
        }
    }

    Ok(())
}

fn update_satellite_position(
    orbits: Query<(&Orbit, &FollowedBy)>,
    mut satellites: Query<(&mut Transform, &Satellite)>,
) -> Result {
    for (orbit, sates) in orbits {
        for sate in &sates.0 {
            let (mut transform, sate) = satellites.get_mut(*sate)?;
            transform.translation = get_pos_from_elements(orbit, sate.mean_anomaly);
        }
    }

    Ok(())
}

pub fn get_pos_from_elements(orbital: &Orbit, mean_anomaly: f32) -> Vec3 {
    let true_anomaly = anomaly_mean_to_true(mean_anomaly, orbital.eccentricity).unwrap();
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
