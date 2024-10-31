use bevy::color::palettes::css::YELLOW;

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
    a: Entity,
    b: Entity,
}

fn setup(mut commands: Commands, satellites: Query<Entity, With<Satellite>>) {
    for satellite in &satellites {
        commands.entity(satellite).insert(Connections {
            connections: Vec::new(),
        });
    }
}

const CONNECTION_DIST: f32 = 2000.0;
const CONNECTION_NUM: usize = 1;
fn connect_nearest(
    mut satellites: Query<(Entity, &Connections, &GlobalTransform), With<Satellite>>,
    mut connections: EventWriter<ConnectTwo>,
) {
    let mut sat_pair = satellites.iter_combinations_mut();
    let mut counter = 0;
    while let Some([(sat1, conn1, trans1), (sat2, conn2, trans2)]) = sat_pair.fetch_next() {
        if counter > 10 {
            break;
        }
        if conn1.connections.len() < CONNECTION_NUM && conn2.connections.len() < CONNECTION_NUM {
            let dist_sq = (trans1.translation() - trans2.translation()).length_squared();

            if dist_sq < CONNECTION_DIST * CONNECTION_DIST {
                connections.send(ConnectTwo { a: sat1, b: sat2 });
                counter += 1;
            }
        }
    }
}

fn handle_connection(
    mut satellites: Query<&mut Connections, With<Satellite>>,
    mut connections: EventReader<ConnectTwo>,
) {
    for ConnectTwo { a, b } in connections.read() {
        let mut conn1 = satellites.get_mut(*a).unwrap();
        conn1.connections.push(*b);
        let mut conn2 = satellites.get_mut(*b).unwrap();
        conn2.connections.push(*a);
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
            gizmos.arrow(start, end, YELLOW);
        }
    }
}
