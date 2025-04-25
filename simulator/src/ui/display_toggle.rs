use super::*;

pub fn spawn_toggle(config: &Config) -> impl Bundle {
    children![
        (
            Node {
                align_items: AlignItems::Center,
                column_gap: Val::Px(5.0),
                ..default()
            },
            children![
                widgets::Toggle(config.Display.orbit),
                (Text::new("Show Orbit"), TextFont::from_font_size(18.0))
            ],
        ),
        (
            Node {
                align_items: AlignItems::Center,
                column_gap: Val::Px(5.0),
                ..default()
            },
            children![
                widgets::Toggle(config.Display.connection),
                (Text::new("Show Connection"), TextFont::from_font_size(18.0))
            ],
        ),
    ]
}

// fn toggle_orbit(
//     trigger: Trigger<widgets::Activate>,
//     toggle: Query<&widgets::Toggle>,
//     mut config: ResMut<Config>,
// ) {
//     config.Display.orbit = toggle.get(trigger.entity()).unwrap().0;
// }

// fn toggle_connection(
//     trigger: Trigger<widgets::Activate>,
//     toggle: Query<&widgets::Toggle>,
//     mut config: ResMut<Config>,
// ) {
//     config.Display.connection = toggle.get(trigger.entity()).unwrap().0;
// }
