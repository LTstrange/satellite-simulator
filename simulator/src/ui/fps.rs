use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

use crate::prelude::*;

#[derive(Component)]
pub struct FpsText;

pub fn fps() -> impl Bundle {
    (
        Text::new("FPS: "),
        TextFont::from_font_size(18.0),
        children![(
            TextSpan::new("0.0"),
            TextColor(GOLD.into()),
            FpsText,
            TextFont::from_font_size(18.0),
        )],
    )
}

pub fn fps_system(diagnostic: Res<DiagnosticsStore>, mut fps: Query<&mut TextSpan, With<FpsText>>) {
    for mut span in &mut fps {
        if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}
