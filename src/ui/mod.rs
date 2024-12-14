use bevy::{
    color::palettes::css::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
};

use crate::prelude::*;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (fps_system, button_system));
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
enum ButtonFunctionality {
    ToggleOrbit,
    ToggleConnection,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

fn setup(mut commands: Commands, config: Res<Config>) {
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            // FPS
            parent
                .spawn((Text::new("FPS: "), TextFont::from_font_size(18.0)))
                .with_child((
                    TextSpan::default(),
                    TextColor(GOLD.into()),
                    FpsText,
                    TextFont::from_font_size(18.0),
                ));

            // Display toggle button
            // Orbit
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    // toggle
                    parent.spawn((
                        Button,
                        Node {
                            height: Val::Percent(80.),
                            aspect_ratio: Some(1.),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(if config.Display.orbit {
                            LIGHT_GRAY.into()
                        } else {
                            NORMAL_BUTTON.into()
                        }),
                        BorderRadius::all(Val::Percent(25.)),
                        BorderColor(BLACK.into()),
                        ButtonFunctionality::ToggleOrbit,
                    ));
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
                    parent.spawn((
                        Button,
                        Node {
                            height: Val::Percent(80.),
                            aspect_ratio: Some(1.),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(if config.Display.orbit {
                            LIGHT_GRAY.into()
                        } else {
                            NORMAL_BUTTON.into()
                        }),
                        BorderRadius::all(Val::Percent(25.)),
                        BorderColor(BLACK.into()),
                        ButtonFunctionality::ToggleConnection,
                    ));
                    // text
                    parent.spawn((Text::new("Show Connection"), TextFont::from_font_size(18.0)));
                });
        });
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

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonFunctionality,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut config: ResMut<Config>,
) {
    for (interaction, mut color, mut border_color, functionality) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = BLACK.into();
                match functionality {
                    ButtonFunctionality::ToggleOrbit => {
                        config.Display.orbit ^= true;

                        *color = if config.Display.orbit {
                            LIGHT_GRAY.into()
                        } else {
                            NORMAL_BUTTON.into()
                        };
                    }
                    ButtonFunctionality::ToggleConnection => {
                        config.Display.connection ^= true;

                        *color = if config.Display.connection {
                            LIGHT_GRAY.into()
                        } else {
                            NORMAL_BUTTON.into()
                        };
                    }
                }
            }
            Interaction::Hovered => {
                border_color.0 = GOLD.into();
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}
