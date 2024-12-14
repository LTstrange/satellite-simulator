use crate::prelude::*;

mod toggle;

pub use toggle::*;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_system);
    }
}
