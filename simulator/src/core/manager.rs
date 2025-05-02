use crate::prelude::*;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // app.init_resource::<SatelliteSpawner>();

        // app.add_event::<SpawnSatellites>();
        // .add_systems(Update, (receive_spawn_event, spawn_satellites).chain())
    }
}

fn setup(mut _commands: Commands) {
    // commands.insert_resource(SatelliteSpawner {
    //     mesh: satellite_mesh,
    //     material: satellite_material,
    //     unspawned_sats: data,
    // });
}

// #[derive(Resource, Default)]
// struct SatelliteSpawner {
//     mesh: Handle<Mesh>,
//     material: Handle<StandardMaterial>,
//     unspawned_sats: Vec<(String, OrbitalElements)>,
// }

// #[derive(Event)]
// pub struct SpawnSatellites {
//     pub satellites: Vec<(String, OrbitalElements)>,
// }

// fn receive_spawn_event(
//     mut event: EventReader<SpawnSatellites>,
//     mut satellite_spawner: ResMut<SatelliteSpawner>,
// ) {
//     for SpawnSatellites { satellites } in event.read() {
//         println!("Receive spawn event: {}", satellites.len());
//         satellite_spawner.unspawned_sats.extend(satellites.clone());
//     }
// }

// fn spawn_satellites(mut commands: Commands, mut satellite_spawner: ResMut<SatelliteSpawner>) {
//     let mesh = satellite_spawner.mesh.clone();
//     let material = satellite_spawner.material.clone();
//     for (satellite_id, satellite) in satellite_spawner.unspawned_sats.drain(..) {
//         assert_ne!(mesh.clone(), Handle::default());
//         assert_ne!(material.clone(), Handle::default());

//         commands.spawn(orbit(
//             satellite_id.to_string(),
//             &satellite,
//             mesh.clone(),
//             material.clone(),
//         ));
//     }
// }
