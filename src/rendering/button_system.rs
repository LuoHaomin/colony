use bevy::prelude::*;
use crate::prelude::*;
use bevy::prelude::NextState;

// Make Plugin
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            button_system
            .run_if(in_state(GameState::MainMenu))
        )
        // .add_system_set(
        //     SystemSet::on_update(GameState::MainMenu)
        //         .with_system(button_system),
        // )
        ;
    }
}


const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// This is for main menu buttons.
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut nextstate: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.0 = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                nextstate.set(GameState::InGame);
            }
            Interaction::Hovered => {
                text.0 = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.0 = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
