use bevy::{
    color::palettes::tailwind::{SKY_700, SLATE_50},
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
};
use fps::fps;

use crate::prelude::*;

mod display_toggle;
mod fps;
// mod widgets;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, fps::fps_system);
    }
}

fn setup(mut commands: Commands, _config: Res<Config>) {
    commands.spawn((
        Node {
            margin: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            parent.spawn(fps());
            parent.spawn(button("Toggle Connection")).observe(
                |_trigger: Trigger<Pointer<Click>>, mut config: ResMut<Config>| {
                    config.display.connection = !config.display.connection;
                    info!("Toggle Connection: {}", config.display.connection);
                },
            );
            parent.spawn(button("Toggle Orbit")).observe(
                |_trigger: Trigger<Pointer<Click>>, mut config: ResMut<Config>| {
                    config.display.orbit = !config.display.orbit;
                    info!("Toggle Orbit: {}", config.display.orbit);
                },
            );
        })),
    ));
}

fn button<T: Into<String>>(text: T) -> impl Bundle {
    (
        Button,
        BackgroundColor(SKY_700.into()),
        Node {
            padding: UiRect::all(Val::Px(5.)),
            // width: Val::Px(200.),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.)),
        children![(Text::new(text), TextColor(SLATE_50.into()))],
    )
}
