use crate::prelude::*;

mod conn_satur_rat;
mod display_toggle;
mod fps;
mod refresh_conn;
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
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            // FPS
            fps::spawn_fps_text(parent);

            // Display toggle button
            display_toggle::spawn_toggle(parent, &config);

            // Refresh connection button
            refresh_conn::spawn_refresh_button(parent);

            // Connection Saturation Rate
            conn_satur_rat::spawn_conn_satur_rate(parent);
        });
}
