use bevy::ecs::entity_disabling::Disabled;

use super::*;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_orbit_gizmo)
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone)]
pub struct Orbit {
    pub mean_motion: f32,                 // 平均运动(rad/s)
    pub eccentricity: f32,                // 离心率
    pub inclination: f32,                 // 轨道倾角(rad)
    pub longitude_of_ascending_node: f32, // 升交点赤经(rad)
    pub argument_of_periapsis: f32,       // 近地点角距(rad)
}

#[derive(Component)]
struct OrbitGizmos;

#[derive(Event)]
pub struct OrbitChanged;

fn setup(mut commands: Commands) {
    commands.spawn((OrbitGizmos, Gizmo::default(), Disabled));
}

fn update_orbit_gizmo(
    _trigger: Trigger<OrbitChanged>,
    gizmos: Option<Single<&mut Gizmo, With<OrbitGizmos>>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    orbits: Query<&Orbit>,
) -> Result {
    info!("update_orbit_gizmo");
    let mut gizmo = GizmoAsset::default();
    for orbit in orbits {
        draw_orbit_gizmo(orbit, &mut gizmo);
    }
    if let Some(mut gizmos) = gizmos {
        gizmos.handle = gizmo_assets.add(gizmo);
    }
    Ok(())
}

fn draw_orbit_gizmo(elements: &Orbit, gizmo: &mut GizmoAsset) {
    // half size of the ellipse
    let n = elements.mean_motion.powf(-2. / 3.);
    // a = u^(1/3) * ( n ) ^ (-2/3)
    let semi_major_axis = FACTOR * n;
    // b = a * sqrt(1 - e^2)
    let semi_minor_axis = semi_major_axis * (1.0 - elements.eccentricity.powi(2)).sqrt();
    let half_size = Vec2::new(semi_major_axis, semi_minor_axis);

    // rotation
    let rotation = get_rotated_quat(
        elements.inclination,
        elements.longitude_of_ascending_node,
        elements.argument_of_periapsis,
    );

    // local position
    // e = c / a; c = e * a
    let semi_focal_distance = semi_major_axis * elements.eccentricity;
    let local_position = Vec3::new(-semi_focal_distance, 0.0, 0.0); // location on the orbital plane
    let location = rotation * local_position; // apply rotation to local position

    let iso = Isometry3d::new(location, rotation);

    gizmo.ellipse(iso, half_size, Color::srgba(1., 1., 1., 0.01));
}
