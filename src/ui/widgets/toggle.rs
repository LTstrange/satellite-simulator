use super::*;

#[derive(Component)]
#[require(Button, Node(node), BorderRadius(|| BorderRadius::all(Val::Percent(25.))), BorderColor(|| Color::BLACK))]
pub struct Toggle(pub bool);

fn node() -> Node {
    Node {
        height: Val::Percent(80.),
        aspect_ratio: Some(1.),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

pub fn toggle_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut Toggle,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        Changed<Interaction>,
    >,
) {
    for (e, interaction, mut toggle, mut bg_color, mut border_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                border_color.0 = Color::BLACK;
                toggle.0 ^= true;
                commands.trigger_targets(Activate, e);
            }
            Interaction::Hovered => {
                border_color.0 = GOLD.into();
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
        bg_color.0 = if toggle.0 { ON } else { OFF };
    }
}
