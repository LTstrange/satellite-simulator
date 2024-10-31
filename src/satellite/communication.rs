use rand::{thread_rng, Rng};

use crate::prelude::*;

pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectTwo>().add_event::<Breaktwo>();
        app.add_systems(Startup, setup.after(super::setup));
        app.add_systems(Update, draw_connections);
        app.add_systems(
            FixedUpdate,
            (
                mark_satellites_try_connect,
                connect_nearest,
                handle_connection,
                break_farthest,
                handle_connection_break,
            ),
        );
    }
}

#[derive(Component)]
struct TryConnect;

#[derive(Component)]
struct Connections {
    connections: Vec<Entity>,
}

#[derive(Event)]
struct ConnectTwo {
    from: Entity,
    to: Entity,
}

#[derive(Event)]
struct Breaktwo {
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

fn mark_satellites_try_connect(
    mut commands: Commands,
    config: Res<Config>,
    satellites: Query<(Entity, &Connections), (With<Satellite>, Without<TryConnect>)>,
) {
    let mut rng = thread_rng();
    let part_of_sats_num = satellites.iter().count() / (config.Simulation.connection_number + 1);
    for sat in satellites
        .iter()
        .filter_map(|(s, c)| {
            // filter out satellites that already saturate their connections
            if c.connections.len() < config.Simulation.connection_number
                && rng.gen::<f32>() < 1. / (config.Simulation.connection_number + 1) as f32
            {
                Some(s)
            } else {
                None
            }
        })
        .take(part_of_sats_num.min(2000))
    {
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
    let connection_num = config.Simulation.connection_number;
    let connection_dist = config.Simulation.connection_distance;
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
            connections.send(ConnectTwo {
                from: cur_sat,
                to: other_sat,
            });
        }
        // remove marker
        commands.entity(cur_sat).remove::<TryConnect>();
    }
}

/// Connect two satellites, based on Connection Events
fn handle_connection(
    config: Res<Config>,
    mut satellites: Query<(Entity, &mut Connections), With<Satellite>>,
    mut connections: EventReader<ConnectTwo>,
) {
    for ConnectTwo { from, to } in connections.read() {
        // println!("Connected {} and {}", from, to);
        let mut to_conn: Mut<'_, Connections> = satellites.get_mut(*to).unwrap().1;
        if to_conn.connections.len() >= config.Simulation.connection_number {
            continue;
        }
        to_conn.connections.push(*from);
        assert!(to_conn.connections.len() <= config.Simulation.connection_number);

        let mut from_conn = satellites.get_mut(*from).unwrap().1;
        from_conn.connections.push(*to);
        assert!(from_conn.connections.len() <= config.Simulation.connection_number);
    }
}

fn break_farthest(
    config: Res<Config>,
    satellites: Query<(Entity, &Connections, &GlobalTransform), With<Satellite>>,
    mut ev_break: EventWriter<Breaktwo>,
) {
    let mut rng = rand::thread_rng();
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
                > config.Simulation.connection_distance * config.Simulation.connection_distance
            {
                ev_break.send(Breaktwo {
                    from: sat,
                    to: other_sat,
                });
            }

            // randomly choose the farthest connections to break
            if conns.connections.len() == config.Simulation.connection_number
                && rng.gen::<f32>() < 1e-4 * config.Simulation.time_speed
            {
                let mut break_sat = None;
                let mut max_distance = 0.0;

                for other_sat in &conns.connections {
                    let other_sat_loc = satellites.get(*other_sat).unwrap().2.translation();
                    let dis_sq = other_sat_loc.distance_squared(cur_loc);
                    if dis_sq > max_distance {
                        max_distance = dis_sq;
                        break_sat = Some(*other_sat);
                    }
                }
                if let Some(break_sat) = break_sat {
                    ev_break.send(Breaktwo {
                        from: sat,
                        to: break_sat,
                    });
                }
            }
        }
    }
}

fn handle_connection_break(
    mut satellites: Query<(Entity, &mut Connections), With<Satellite>>,
    mut connections: EventReader<Breaktwo>,
) {
    for Breaktwo { from, to } in connections.read() {
        let mut from_conn = satellites.get_mut(*from).unwrap().1;
        from_conn.connections.retain(|&sat| sat != *to);

        let mut to_conn = satellites.get_mut(*to).unwrap().1;
        to_conn.connections.retain(|&sat| sat != *from);
    }
}

/// GIZMOS

fn draw_connections(
    config: Res<Config>,
    mut gizmos: Gizmos,
    satellites: Query<(Entity, &Connections, &GlobalTransform), With<Satellite>>,
) {
    if !config.Display.connection {
        return;
    }
    for (_, connections, global_trans) in &satellites {
        let start = global_trans.translation();
        for other_sat in &connections.connections {
            let end = satellites.get(*other_sat).unwrap().2.translation();
            gizmos.arrow(
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
