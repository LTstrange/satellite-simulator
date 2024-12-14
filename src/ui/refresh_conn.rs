use bevy::color::palettes::css::*;

use crate::prelude::*;

#[derive(Component)]
pub struct RefreshButton;

pub fn spawn_refresh_button(parent: &mut ChildBuilder) {
    // Clear all connections
    parent
        .spawn((
            Button,
            Node {
                border: UiRect::all(Val::Px(2.0)),
                // margin: UiRect::all(Val::Percent(50.)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BorderRadius::all(Val::Percent(25.)),
            BorderColor(BLACK.into()),
            RefreshButton,
        ))
        .with_child((
            Text::new("Refresh Connections"),
            TextFont::from_font_size(18.0),
        ));
}

pub fn refresh_button_system() {}
