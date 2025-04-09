use super::*;

#[derive(Component)]
pub struct EllipseOrbitData {
    location: Vec3,
    rotation: Quat,
    half_size: Vec2,
}

pub fn get_ellipse_orbit_data(satellite: &OrbitalElements) -> EllipseOrbitData {
    // half size of the ellipse

    let n = satellite.mean_motion.powf(-2. / 3.);
    // a = u^(1/3) * ( n ) ^ (-2/3)
    let semi_major_axis = FACTOR * n;
    // b = a * sqrt(1 - e^2)
    let semi_minor_axis = semi_major_axis * (1.0 - satellite.eccentricity.powi(2)).sqrt();
    let half_size = Vec2::new(semi_major_axis, semi_minor_axis);

    let rotation = get_rotated_quat(
        satellite.inclination,
        satellite.longitude_of_ascending_node,
        satellite.argument_of_periapsis,
    );

    // local position
    // e = c / a; c = e * a
    let semi_focal_distance = semi_major_axis * satellite.eccentricity;
    let local_position = Vec3::new(-semi_focal_distance, 0.0, 0.0); // location on the orbital plane
    let location = rotation * local_position; // apply rotation to local position
    EllipseOrbitData {
        location,
        rotation,
        half_size,
    }
}

// Gizmos
pub fn draw_ellipse_orbit(
    config: Res<Config>,
    mut gizmos: Gizmos,
    query: Query<&EllipseOrbitData>,
) {
    if !config.Display.orbit {
        return;
    }

    for ellpise in &query {
        let iso = Isometry3d::new(ellpise.location, ellpise.rotation);
        gizmos.ellipse(iso, ellpise.half_size, Color::srgba(1., 1., 1., 0.01));
    }
}
