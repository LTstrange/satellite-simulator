use display_toggle::spawn_toggle;
use fps::fps_text;

use crate::prelude::*;

mod display_toggle;
mod fps;
mod widgets;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(widgets::WidgetsPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, fps::fps_system);
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        children![
            // FPS
            fps_text(),
            //Display toggle button
            spawn_toggle(&config),
        ],
    ));
}
