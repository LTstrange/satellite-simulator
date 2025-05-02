use super::*;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SatelliteManager>();

        app.add_event::<SpawnSatellites>()
            .add_event::<SpawnOrbits>()
            .add_event::<AttachSatellites>()
            .add_systems(FixedUpdate, (receive_events, spawn).chain());
    }
}

#[derive(Resource, Default)]
pub struct SatelliteManager {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    orbit_entities: Vec<Entity>,
    unspawned_sats: Vec<(String, OrbitalElements)>,
    unspawned_orbs: Vec<Orbit>,
    unattached_sats: Vec<(Entity, String, Satellite)>,
}

impl SatelliteManager {
    pub fn init(&mut self, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) {
        self.mesh = mesh;
        self.material = material;
    }

    pub fn add_satellites(&mut self, satellites: Vec<(String, OrbitalElements)>) {
        self.unspawned_sats.extend(satellites);
    }
}

#[derive(Event)]
pub struct SpawnSatellites {
    satellites: Vec<(String, OrbitalElements)>,
}

#[derive(Event)]
pub struct SpawnOrbits {
    pub orbits: Vec<Orbit>,
}

#[derive(Event)]
pub struct AttachSatellites {
    pub satellites: Vec<(Entity, String, Satellite)>,
}

fn receive_events(
    mut spawn_sat_events: EventReader<SpawnSatellites>,
    mut spawn_orbit_events: EventReader<SpawnOrbits>,
    mut attach_sat_events: EventReader<AttachSatellites>,
    mut satellite_spawner: ResMut<SatelliteManager>,
) {
    satellite_spawner.unspawned_sats.extend(
        spawn_sat_events
            .read()
            .map(|event| event.satellites.clone())
            .flatten(),
    );
    satellite_spawner.unspawned_orbs.extend(
        spawn_orbit_events
            .read()
            .map(|event| event.orbits.clone())
            .flatten(),
    );
    satellite_spawner.unattached_sats.extend(
        attach_sat_events
            .read()
            .map(|event| event.satellites.clone())
            .flatten(),
    );
}

fn spawn(mut commands: Commands, mut satellite_spawner: ResMut<SatelliteManager>) {
    let mesh = satellite_spawner.mesh.clone();
    let material = satellite_spawner.material.clone();
    assert_ne!(mesh.clone(), Handle::default());
    assert_ne!(material.clone(), Handle::default());

    let mut need_update_orbit_gizmo = false;

    // spawn orbits
    let iter: Vec<_> = satellite_spawner
        .unspawned_orbs
        .drain(..)
        .map(|orbit| {
            commands.spawn(orbit).id()
            // satellite_spawner.orbit_entities.push(orbit_entity);
        })
        .collect();
    if iter.len() > 0 {
        need_update_orbit_gizmo = true;
    }
    satellite_spawner.orbit_entities.extend(iter);

    // spawn satellites
    let iter: Vec<_> = satellite_spawner
        .unspawned_sats
        .drain(..)
        .map(|elements| {
            let (satellite_id, satellite) = elements;
            let (orbit, mean_anomaly) = satellite.sep_out_mean_anomaly();
            let orbit_entity = commands.spawn(orbit).id();
            commands.spawn(create_satellite(
                satellite_id,
                orbit_entity,
                mean_anomaly,
                mesh.clone(),
                material.clone(),
            ));
            orbit_entity
        })
        .collect();
    if iter.len() > 0 {
        need_update_orbit_gizmo = true;
    }
    satellite_spawner.orbit_entities.extend(iter);

    // attach satellites to orbits
    satellite_spawner.unattached_sats.drain(..).for_each(
        |(orbit_entity, satellite_id, satellite)| {
            commands.spawn(create_satellite(
                satellite_id,
                orbit_entity,
                satellite.mean_anomaly,
                mesh.clone(),
                material.clone(),
            ));
        },
    );

    if need_update_orbit_gizmo {
        info!("Send out OrbitChanged event");
        commands.trigger(OrbitChanged);
    }
}
