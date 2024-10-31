use super::*;

#[derive(Component)]
pub struct EllipseOrbitData {
    location: Vec3,
    rotation: Quat,
    half_size: Vec2,
}

pub fn setup_ellipse_orbit_data(mut commands: Commands, orbits: Query<(Entity, &OrbitalElements)>) {
    for (entity, element) in &orbits {
        // half size of the ellipse

        let n = element.mean_motion.powf(-2. / 3.);
        // a = u^(1/3) * ( n ) ^ (-2/3)
        let semi_major_axis = FACTOR * n;
        // b = a * sqrt(1 - e^2)
        let semi_minor_axis = semi_major_axis * (1.0 - element.eccentricity.powi(2)).sqrt();
        let half_size = Vec2::new(semi_minor_axis, semi_major_axis);

        let rotation = get_rotated_quat(
            element.inclination,
            element.longitude_of_ascending_node,
            element.argument_of_periapsis,
        );

        // local position
        // e = c / a; c = e * a
        let semi_focal_distance = semi_major_axis * element.eccentricity;
        let local_position = Vec3::new(-semi_focal_distance, 0.0, 0.0); // location on the orbital plane
        let location = rotation * local_position; // apply rotation to local position
        commands.entity(entity).insert(EllipseOrbitData {
            location,
            rotation,
            half_size,
        });
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
        gizmos.ellipse(
            ellpise.location,
            ellpise.rotation,
            ellpise.half_size,
            Color::srgba(1., 1., 1., 0.01),
        );
    }
}
