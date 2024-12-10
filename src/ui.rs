use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

use crate::prelude::*;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, fps_system);
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    // FPS
    commands
        .spawn((Text::new("FPS: "), TextFont::from_font_size(12.0)))
        .with_child((
            TextSpan::default(),
            TextColor(GOLD.into()),
            FpsText,
            TextFont::from_font_size(12.0),
        ));
}

fn fps_system(diagnostic: Res<DiagnosticsStore>, mut fps: Query<&mut TextSpan, With<FpsText>>) {
    for mut span in &mut fps {
        if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}
