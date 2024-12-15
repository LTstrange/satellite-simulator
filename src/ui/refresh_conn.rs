use crate::prelude::*;

use super::widgets::*;

pub fn spawn_refresh_button(parent: &mut ChildBuilder) {
    // Clear all connections
    parent
        .spawn(CusButton)
        .observe(refresh_connections)
        .with_child((
            Text::new("Refresh Connections"),
            TextFont::from_font_size(18.0),
        ));
}

fn refresh_connections(trigger: Trigger<Activate>) {}
