use crate::prelude::*;

mod display_toggle;
mod fps;
mod refresh_conn;
mod widgets;

use widgets::*;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                fps::fps_system,
                toggle_system,
                refresh_conn::refresh_button_system,
            ),
        );
    }
}

fn setup(mut commands: Commands, config: Res<Config>) {
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent: &mut ChildBuilder<'_>| {
            // FPS
            fps::spawn_fps_text(parent);

            // Display toggle button
            display_toggle::spawn_toggle(parent, &config);

            // Refresh connection button
            refresh_conn::spawn_refresh_button(parent);
        });
}
