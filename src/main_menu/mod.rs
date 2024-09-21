use bevy::prelude::*;
mod components;
mod styles;
pub mod systems;
use crate::states::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            systems::layout::spawn_main_menu,
        );
        app.add_systems(
            OnExit(AppState::MainMenu),
            systems::layout::despawn_main_menu,
        );
        app.add_plugins(systems::interactions::ButtonPlugin);
    }
}
