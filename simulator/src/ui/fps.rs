use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

use crate::prelude::*;

#[derive(Component)]
pub struct FpsText;

pub fn spawn_fps_text(parent: &mut ChildBuilder) {
    parent
        .spawn((Text::new("FPS: "), TextFont::from_font_size(18.0)))
        .with_child((
            TextSpan::default(),
            TextColor(GOLD.into()),
            FpsText,
            TextFont::from_font_size(18.0),
        ));
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
