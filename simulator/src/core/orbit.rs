use bevy::ecs::entity_disabling::Disabled;

use super::*;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_orbit_gizmos)
            .add_observer(toggle_orbit_gizmos)
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

#[derive(Event)]
pub struct ToggleOrbitGizmos;

fn setup(mut commands: Commands) {
    commands.spawn((OrbitGizmos, Gizmo::default()));
}

fn update_orbit_gizmos(
    _trigger: Trigger<OrbitChanged>,
    gizmos: Single<(&mut Gizmo, Has<Disabled>), With<OrbitGizmos>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    orbits: Query<&Orbit>,
) {
    info!("update_orbit_gizmo");
    let mut gizmo = GizmoAsset::default();
    for orbit in orbits {
        draw_orbit_gizmo(orbit, &mut gizmo);
    }
    let (mut gizmos, _) = gizmos.into_inner();
    gizmos.handle = gizmo_assets.add(gizmo);
}

fn toggle_orbit_gizmos(
    _trigger: Trigger<ToggleOrbitGizmos>,
    gizmos: Single<(Entity, Has<Disabled>), With<OrbitGizmos>>,
    config: Res<Config>,
    mut commands: Commands,
) {
    let (e, _) = gizmos.into_inner();
    if config.display.orbit {
        commands.entity(e).remove::<Disabled>();
    } else {
        commands.entity(e).insert(Disabled);
    }
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
