use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
