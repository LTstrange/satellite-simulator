use super::*;

#[derive(Component)]
#[require(Button, Node(node), BorderRadius(|| BorderRadius::all(Val::Percent(25.))), BorderColor(|| Color::BLACK), BackgroundColor(|| OFF))]
pub struct CusButton;

fn node() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        border: UiRect::all(Val::Px(2.0)),
        // margin: UiRect::all(Val::Percent(50.)),
        padding: UiRect::all(Val::Px(10.0)),
        ..default()
    }
}

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<CusButton>),
    >,
) {
    for (e, interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                border_color.0 = BLACK.into();
                bg_color.0 = ON.into();
                commands.trigger_targets(Activate, e);
            }
            Interaction::Hovered => {
                border_color.0 = GOLD.into();
                bg_color.0 = OFF.into();
            }
            Interaction::None => {
                border_color.0 = BLACK.into();
                bg_color.0 = OFF.into();
            }
        }
    }
}
