use crate::prelude::*;

pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectTwo>();
        app.add_systems(Startup, setup.after(super::setup));
        app.add_systems(Update, draw_connections);
        app.add_systems(FixedUpdate, (connect_nearest, handle_connection));
    }
}

#[derive(Component)]
struct Connections {
    connections: Vec<Entity>,
}

#[derive(Event)]
struct ConnectTwo {
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

const CONNECTION_DIST: f32 = 6000.0;
const CONNECTION_NUM: usize = 4;
fn connect_nearest(
    satellites: Query<(Entity, &Connections, &GlobalTransform), With<Satellite>>,
    mut connections: EventWriter<ConnectTwo>,
) {
    let mut satellies_iter = satellites
        .into_iter()
        // filter out satellites that already saturate their connections
        .filter(|(_, conn, _)| conn.connections.len() < CONNECTION_NUM)
        // global transform to global coordinates
        .map(|(sat, conn, trans)| (sat, conn, trans.translation()));

    // find the first satellite and its connections
    if let Some((cur_sat, cur_conn, cur_pos)) = satellies_iter.next() {
        // get all other satellites within the connection distance
        let mut other_satellites: Vec<_> = satellies_iter
            .map(|(s, c, t)| (s, c, t.distance_squared(cur_pos)))
            .filter(|(_, _, t)| *t < CONNECTION_DIST * CONNECTION_DIST)
            .collect();
        // sort by distance to the current satellite
        other_satellites.sort_unstable_by(|a, b| a.2.total_cmp(&b.2));

        let count = CONNECTION_NUM - cur_conn.connections.len();
        for (other_sat, _, _) in &other_satellites[..count.min(other_satellites.len())] {
            connections.send(ConnectTwo {
                from: cur_sat,
                to: *other_sat,
            });
        }
    }
}

fn handle_connection(
    mut satellites: Query<(Entity, &mut Connections), With<Satellite>>,
    mut connections: EventReader<ConnectTwo>,
) {
    for ConnectTwo { from, to } in connections.read() {
        let mut from_conn = satellites.get_mut(*from).unwrap().1;
        from_conn.connections.push(*to);
        let mut to_conn = satellites.get_mut(*to).unwrap().1;
        to_conn.connections.push(*from);
    }
}

/// GIZMOS

fn draw_connections(
    mut gizmos: Gizmos,
    satellites: Query<(Entity, &GlobalTransform, &Connections), With<Satellite>>,
) {
    for (_, global_trans, connections) in &satellites {
        let start = global_trans.translation();
        for other_sat in &connections.connections {
            let end = satellites.get(*other_sat).unwrap().1.translation();
            gizmos.arrow(
                start,
                end,
                Srgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 0.5,
                },
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter() {
        let sats = [1, 2, 3, 4, 5, 6];
        let mut iter = sats.into_iter();
        let first = iter.next().unwrap();
        let rest = iter.collect::<Vec<_>>();

        println!("first: {}, rest: {:?}", first, rest); // first: 1, rest: [2, 3, 4, 5, 6]
    }
}
