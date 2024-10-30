use bevy::color::palettes::css::YELLOW;

use crate::prelude::*;

pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectionRequest>()
            .add_event::<ConnectionResponse>()
            .add_event::<SatelliteNeedConnection>();
        app.add_systems(Startup, setup.after(super::setup));
        app.add_systems(Update, draw_connections);
        app.add_systems(
            FixedUpdate,
            (
                trigger_connection_request,
                request_connection_nearest,
                response_connection,
                handle_response,
            ),
        );
    }
}

#[derive(Component)]
struct Connections {
    connections: Vec<Entity>,
}

#[derive(Event)]
struct SatelliteNeedConnection {
    satellite: Entity,
    global_transform: Vec3,
    lack: u8,
}

#[derive(Event)]
struct ConnectionRequest {
    from: Entity,
    to: Entity,
}

#[derive(Event)]
struct ConnectionResponse {
    from: Entity,
    to: Entity,
}

fn setup(mut commands: Commands, satellites: Query<Entity, With<Satellite>>) {
    for satellite in &satellites {
        commands.entity(satellite).insert(Connections {
            connections: Vec::new(),
        });
    }
}

/// find all satellites that need to request a connection
/// and send (max_conn - existing_connections) times trigger events
fn trigger_connection_request(
    satellites: Query<(Entity, &GlobalTransform, &Connections), With<Satellite>>,
    mut ev_sat_need_conn: EventWriter<SatelliteNeedConnection>,
) {
    for (satellite, global_trans, conn) in &satellites {
        if conn.connections.len() < 4 {
            ev_sat_need_conn.send(SatelliteNeedConnection {
                satellite,
                global_transform: global_trans.translation(),
                lack: 4 - conn.connections.len() as u8,
            });
        }
    }
}

fn request_connection_nearest(
    mut ev_sat_need_conn: EventReader<SatelliteNeedConnection>,
    mut ev_conn_req: EventWriter<ConnectionRequest>,
) {
    let satellites: Vec<(Entity, Vec3, u8)> = ev_sat_need_conn
        .read()
        .map(
            |SatelliteNeedConnection {
                 satellite,
                 global_transform,
                 lack,
             }| (*satellite, *global_transform, *lack),
        )
        .collect();

    for (satellite, global_trans, lack) in &satellites {
        let mut other_sat_rel = satellites
            .iter()
            .filter_map(|(e, t, _)| {
                if e != satellite {
                    Some((*e, t.distance_squared(*global_trans)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        other_sat_rel.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));

        for (other_sat, _) in &other_sat_rel[..(*lack as usize)] {
            ev_conn_req.send(ConnectionRequest {
                from: *satellite,
                to: *other_sat,
            });
        }
    }
}

fn response_connection(
    mut satellites: Query<&mut Connections, With<Satellite>>,
    mut ev_conn_req: EventReader<ConnectionRequest>,
    mut ev_conn_resp: EventWriter<ConnectionResponse>,
) {
    for ConnectionRequest { from, to } in ev_conn_req.read() {
        let mut connections = satellites.get_mut(*to).unwrap();
        if connections.connections.len() < 4 {
            connections.connections.push(*from);
            ev_conn_resp.send(ConnectionResponse {
                from: *to,
                to: *from,
            });
        }
    }
}

fn handle_response(
    mut ev_conn_resp: EventReader<ConnectionResponse>,
    mut satellites: Query<&mut Connections, With<Satellite>>,
) {
    for ConnectionResponse { from, to } in ev_conn_resp.read() {
        let mut connections = satellites.get_mut(*from).unwrap();
        connections.connections.push(*from);
    }
}

fn draw_connections(
    mut gizmos: Gizmos,
    satellites: Query<(Entity, &GlobalTransform, &Connections), With<Satellite>>,
) {
    for (_, global_trans, connections) in &satellites {
        let start = global_trans.translation();
        for other_sat in &connections.connections {
            let end = satellites.get(*other_sat).unwrap().1.translation();
            gizmos.arrow(start, end, YELLOW);
            println!("connection: {} -> {}", start, end);
        }
    }
}
