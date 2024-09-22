#![allow(dead_code, unused_imports, unused_variables)]
use bevy::ecs::schedule::ExecutorKind;
use bevy::window::PrimaryWindow;
use bevy::{input::keyboard, prelude::*};

use bevy::reflect::TypeRegistry;

mod player;

mod map;

mod main_menu;

mod states;
use states::AppState::{self, *};

mod camera;

mod procedural_generation;

fn main() {
    bevy::app::App::new()
        //
        // === Plugins ===
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_linear()))
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(procedural_generation::ProceduralGenerationPlugin)
        //
        // === Resources ===
        .insert_state(MainMenu)
        .init_state::<DespawnedYet>()
        //
        // === Systems ===
        .add_systems(Update, switch_state)
        .run();
}

fn switch_state(
    mut state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut despawned: ResMut<NextState<DespawnedYet>>,
) {
    // Escape Toggles between MainMenu and InGame
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            MainMenu => state.set(InGame),
            InGame => {
                despawned.set(DespawnedYet::Yes);
                state.set(MainMenu);
            }
            _ => {}
        }
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum DespawnedYet {
    #[default]
    No,
    Yes,
}

#[derive(Component)]
pub struct Active;

fn movement_keys(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::KeyS)
        || keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::KeyD)
}
