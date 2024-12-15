use crate::prelude::*;
use bevy::color::palettes::css::*;

mod button;
mod toggle;

pub use button::*;
pub use toggle::*;

const OFF: Color = Color::srgb(0.15, 0.15, 0.15);
const ON: Color = Color::srgb(0.827, 0.827, 0.827);

#[derive(Event)]
pub struct Activate;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_system, button_system));
    }
}
