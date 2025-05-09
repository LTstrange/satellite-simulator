use rand::{rng, seq::SliceRandom};

use super::satellite::Satellite;
use crate::prelude::*;

pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<ConnectTwo>()
            .add_event::<DisconnectTwo>()
            .add_event::<DisconnectAll>();

        // Gizmos for visualization
        app.add_systems(Update, draw_connections);

        // Functionality
        app.add_systems(
            FixedUpdate,
            (
                mark_satellites_try_connect,
                connect_nearest,
                handle_connection,
                disconnect_farthest,
                handle_disconnection,
            ),
        );

        app.add_systems(
            FixedUpdate,
            handle_disconnect_all.after(mark_satellites_try_connect),
        );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct TryConnect;

#[derive(Component, Default)]
pub struct Connections {
    connections: Vec<Entity>,
}

#[derive(Event)]
struct ConnectTwo {
    from: Entity,
    to: Entity,
}

#[derive(Event)]
struct DisconnectTwo {
    from: Entity,
    to: Entity,
}

#[derive(Event)]
pub struct DisconnectAll;

fn mark_satellites_try_connect(
    mut commands: Commands,
    config: Res<Config>,
    satellites: Query<(Entity, &Connections), (With<Satellite>, Without<TryConnect>)>,
) {
    let mut rng = rng();

    // debug
    // let empty_sats = satellites
    //     .iter()
    //     .filter(|(_, c)| c.connections.len() == 0)
    //     .collect::<Vec<_>>();
    // if empty_sats.len() != 0 {
    //     info!("Empty satellites: {:?}", empty_sats.len());
    // }

    // filter out satellites that already saturate their connections
    let mut unfull_satellites = satellites
        .iter()
        .filter_map(|(s, c)| {
            // filter out satellites that already saturate their connections
            if c.connections.len() < config.simulation.connection_number {
                Some((s, c.connections.len()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // try to make sure one sat could connect to `connection_number` satellites
    let part_of_unfull_sats_num =
        unfull_satellites.len() / (config.simulation.connection_number + 1);

    unfull_satellites.shuffle(&mut rng); // O(n)
    unfull_satellites.sort_unstable_by_key(|(_, c)| *c); // O(n * log(connection_number)) ~ O(n)
    for &(sat, _) in unfull_satellites.iter().take(part_of_unfull_sats_num) {
        commands.entity(sat).insert(TryConnect);
    }
}

fn connect_nearest(
    config: Res<Config>,
    mut commands: Commands,
    from_satellites: Query<
        (Entity, &Connections, &GlobalTransform),
        (With<Satellite>, With<TryConnect>),
    >,
    to_satellites: Query<
        (Entity, &Connections, &GlobalTransform),
        (With<Satellite>, Without<TryConnect>),
    >,
    mut connections: EventWriter<ConnectTwo>,
) {
    // get configuration parameters
    let connection_num = config.simulation.connection_number;
    let connection_dist = config.simulation.connection_distance;

    // get all sats which are trying to connect, and get their global positions
    let from_sats_iter = from_satellites
        .into_iter()
        // global transform to global coordinates
        .map(|(sat, conn, trans)| (sat, conn, trans.translation()));

    let to_satellies_iter = to_satellites
        .into_iter()
        // filter out satellites that already saturate their connections
        .filter(|(_, conn, _)| conn.connections.len() < connection_num)
        // global transform to global coordinates
        .map(|(sat, conn, trans)| (sat, conn, trans.translation()))
        .collect::<Vec<_>>();

    // find the first satellite and its connections
    for (cur_sat, cur_conn, cur_pos) in from_sats_iter {
        // get all other satellites within the connection distance
        let mut other_satellites: Vec<_> = to_satellies_iter
            .iter()
            .map(|(s, c, t)| (s, c, t.distance_squared(cur_pos)))
            .filter(|(_, _, t)| *t < connection_dist * connection_dist)
            .collect();
        // sort by distance to the current satellite
        other_satellites.sort_unstable_by(|a, b| a.2.total_cmp(&b.2));

        let count = connection_num - cur_conn.connections.len();
        for (&other_sat, _, _) in &other_satellites[..count.min(other_satellites.len())] {
            connections.write(ConnectTwo {
                from: cur_sat,
                to: other_sat,
            });
        }
        // remove marker
        commands.entity(cur_sat).remove::<TryConnect>();
    }
}

fn disconnect_farthest(
    config: Res<Config>,
    satellites: Query<(Entity, &Connections, &GlobalTransform), With<Satellite>>,
    mut ev_break: EventWriter<DisconnectTwo>,
) {
    let mut batch = vec![];
    for (sat, conns, trans) in &satellites {
        let cur_loc = trans.translation();
        for other_sat in conns.connections.clone() {
            // guarantee not to break the same connection twice
            if sat > other_sat {
                continue;
            }

            let other_loc = satellites.get(other_sat).unwrap().2.translation();
            let dis_sq = other_loc.distance_squared(cur_loc);
            // break the connection which exceeds the connection distance
            if dis_sq
                > config.simulation.connection_distance * config.simulation.connection_distance
            {
                // ev_break.write(DisconnectTwo {
                //     from: sat,
                //     to: other_sat,
                // });
                batch.push(DisconnectTwo {
                    from: sat,
                    to: other_sat,
                });
            }
        }
    }

    ev_break.write_batch(batch);
}

// --------------- Handle Events ---------------

/// Connect two satellites, based on Connection Events
fn handle_connection(
    config: Res<Config>,
    mut satellites: Query<(Entity, &mut Connections), With<Satellite>>,
    mut connections: EventReader<ConnectTwo>,
) {
    for ConnectTwo { from, to } in connections.read() {
        // println!("Connected {} and {}", from, to);
        let mut to_conn: Mut<'_, Connections> = satellites.get_mut(*to).unwrap().1;
        if to_conn.connections.len() >= config.simulation.connection_number {
            continue;
        }
        to_conn.connections.push(*from);
        assert!(to_conn.connections.len() <= config.simulation.connection_number);

        let mut from_conn = satellites.get_mut(*from).unwrap().1;
        from_conn.connections.push(*to);
        assert!(from_conn.connections.len() <= config.simulation.connection_number);
    }
}

/// Disconnect two satellites, based on Disconnection Events
fn handle_disconnection(
    mut satellites: Query<(Entity, &mut Connections), With<Satellite>>,
    mut connections: EventReader<DisconnectTwo>,
) {
    for DisconnectTwo { from, to } in connections.read() {
        let mut from_conn = satellites.get_mut(*from).unwrap().1;
        from_conn.connections.retain(|&sat| sat != *to);

        let mut to_conn = satellites.get_mut(*to).unwrap().1;
        to_conn.connections.retain(|&sat| sat != *from);
    }
}

fn handle_disconnect_all(
    mut commands: Commands,
    mut e: EventReader<DisconnectAll>,
    mut satellites: Query<(Entity, &mut Connections), With<Satellite>>,
) {
    for _ in e.read() {
        // println!("Disconnecting all satellites");
        for (sat, mut conns) in &mut satellites {
            conns.connections.clear();
            commands.entity(sat).remove::<TryConnect>();
        }
    }
}

// --------------- GIZMOS ---------------

fn draw_connections(
    config: Res<Config>,
    mut gizmos: Gizmos,
    satellites: Query<(Entity, &Connections, &GlobalTransform), With<Satellite>>,
) {
    if !config.display.connection {
        return;
    }
    for (e, connections, global_trans) in &satellites {
        let start = global_trans.translation();
        for other_sat in &connections.connections {
            if e > *other_sat {
                continue;
            }
            let end = satellites.get(*other_sat).unwrap().2.translation();
            gizmos.line(
                start,
                end,
                Srgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 0.2,
                },
            );
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_iter() {
        let sats = [1, 2, 3, 4, 5, 6];
        let mut iter = sats.into_iter();
        let first = iter.next().unwrap();
        let rest = iter.collect::<Vec<_>>();

        println!("first: {}, rest: {:?}", first, rest); // first: 1, rest: [2, 3, 4, 5, 6]
    }
}
