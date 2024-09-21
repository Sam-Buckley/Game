use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use crate::states::AppState;

pub fn play_button_system(
    //mut commands: Commands,
    mut button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut state: ResMut<NextState<AppState>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok((interaction, mut background_color)) = button.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_PRESSED_COLOR.into();
                // Wait for the button to be released before changing the state
            }
            Interaction::Hovered => {
                *background_color = BUTTON_HOVER_COLOR.into();
                // INFO - When the mouse is released, Bevy views the button as hovered
                if mouse_input.just_released(MouseButton::Left) {
                    state.set(AppState::InGame);
                }
            }
            Interaction::None => {
                // Light blue
                *background_color = BUTTON_COLOR.into();
            }
        }
    }
}

fn quit_button_system(
    //mut commands: Commands,
    mut button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok((interaction, mut background_color)) = button.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_PRESSED_COLOR.into();
                // Wait for the button to be released before changing the state
            }
            Interaction::Hovered => {
                *background_color = BUTTON_HOVER_COLOR.into();
                // INFO - When the mouse is released, Bevy views the button as hovered
                if mouse_input.just_released(MouseButton::Left) {
                    std::process::exit(0);
                }
            }
            Interaction::None => {
                // Light blue
                *background_color = BUTTON_COLOR.into();
            }
        }
    }
}
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                play_button_system.run_if(in_state(AppState::MainMenu)),
                quit_button_system.run_if(in_state(AppState::MainMenu)),
            ),
        );
    }
}
