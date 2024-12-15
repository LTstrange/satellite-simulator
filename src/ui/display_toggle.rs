use super::*;

pub fn spawn_toggle(parent: &mut ChildBuilder, config: &Config) {
    // Orbit
    parent
        .spawn(Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|parent| {
            // toggle
            parent
                .spawn(widgets::Toggle(config.Display.orbit))
                .observe(toggle_orbit);
            // text
            parent.spawn((Text::new("Show Orbit"), TextFont::from_font_size(18.0)));
        });

    // Connection
    parent
        .spawn(Node {
            align_items: AlignItems::Center,
            column_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|parent| {
            // toggle
            parent
                .spawn(widgets::Toggle(config.Display.connection))
                .observe(toggle_connection);
            // text
            parent.spawn((Text::new("Show Connection"), TextFont::from_font_size(18.0)));
        });
}

fn toggle_orbit(
    trigger: Trigger<widgets::Activate>,
    toggle: Query<&widgets::Toggle>,
    mut config: ResMut<Config>,
) {
    config.Display.orbit = toggle.get(trigger.entity()).unwrap().0;
}

fn toggle_connection(
    trigger: Trigger<widgets::Activate>,
    toggle: Query<&widgets::Toggle>,
    mut config: ResMut<Config>,
) {
    config.Display.connection = toggle.get(trigger.entity()).unwrap().0;
}
