#![allow(dead_code, unused_imports, unused_variables)]
use bevy::window::PrimaryWindow;
use bevy::{input::keyboard, prelude::*};

mod main_menu;

mod states;
use states::AppState::{self, *};

mod camera;

fn main() {
    bevy::app::App::new()
        //
        // === Plugins ===
        .add_plugins(DefaultPlugins)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(camera::CameraPlugin)
        //
        // === Resources ===
        .insert_state(MainMenu)
        //
        // === Systems ===
        .add_systems(Startup, spawn_background)
        .add_systems(Update, switch_state)
        .run();
}

fn switch_state(
    mut state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    // Escape Toggles between MainMenu and InGame
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            MainMenu => state.set(InGame),
            InGame => state.set(MainMenu),
            _ => {}
        }
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();
    let transform =
        Transform::from_translation(Vec3::new(window_width / 2.0, window_height / 2.0, 0.0))
            .with_scale(Vec3::splat(1.3));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/Mountains.png"),
        transform,
        ..Default::default()
    });
}
