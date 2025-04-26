use bevy::ecs::{relationship::RelatedSpawner, spawn::SpawnWith};
use display_toggle::toggle;
use fps::fps;

use crate::prelude::*;

mod display_toggle;
mod fps;
mod widgets;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, fps::fps_system);
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    let orbit = config.Display.orbit;
    let connection = config.Display.connection;

    commands.spawn((
        Node {
            margin: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..default()
        },
        children![
            fps(),
            // toggle("Show Orbit", orbit),
            // toggle("Show Connection", connection)
        ], // Children::spawn((
           //     Spawn(fps()),
           //     SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
           //         parent.spawn(toggle("Show Orbit", orbit)).observe(
           //             |trigger: Trigger<widgets::Activate>,
           //              toggle: Query<&widgets::Toggle>,
           //              mut config: ResMut<Config>| {
           //                 config.Display.orbit = toggle.get(trigger.target()).unwrap().0;
           //             },
           //         );
           //         parent.spawn(toggle("Show Connection", connection)).observe(
           //             |trigger: Trigger<widgets::Activate>,
           //              toggle: Query<&widgets::Toggle>,
           //              mut config: ResMut<Config>| {
           //                 config.Display.connection = toggle.get(trigger.target()).unwrap().0;
           //             },
           //         );
           //     }),
           // )),
    ));
}
