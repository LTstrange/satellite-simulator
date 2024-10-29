use super::*;

/// Read and Setup satellite data and add them to the scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let data = File::open("./FENGYUN.json").unwrap();
    let satellites: Vec<SatelliteData> = serde_json::from_reader(data).unwrap();

    let satellite_mesh = meshes.add(Sphere::new(20.).mesh().ico(1).unwrap());
    let satellite_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        unlit: true,
        ..default()
    });

    let current_time = Utc::now();
    const index: usize = 1;
    for satellite in &satellites[..] {
        let observe_time = parse_time_from_str(&satellite.EPOCH);

        let duration = current_time - observe_time.unwrap();

        let mut orbital = OrbitalElements::from(satellite.clone());
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

pub fn update_mean_anomaly(
    mut satellites: Query<&mut OrbitalElements, With<Satellite>>,
    time: Res<Time<Fixed>>,
) {
    for mut element in &mut satellites {
        element.mean_anomaly += element.mean_motion * time.delta_seconds() * 100.;
        element.mean_anomaly %= 2. * PI;
    }
}

pub fn update_satellite_position(mut satellites: Query<(&mut Transform, &OrbitalElements)>) {
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
