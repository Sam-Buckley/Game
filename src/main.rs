#![allow(dead_code)]
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use loss::LossEvent;
mod hero;
mod camera;
mod background;
mod enemy;
mod loss;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // Window descriptor to open fullscreen
            WindowPlugin {
                primary_window: Some(
                    Window {
                        title: "Ball Game".to_string(),
                        resolution: (1920., 1080.).into(),
                        ..Default::default()
                    }),
                ..Default::default()
            },
        ))
        .add_plugins((camera::CameraPlugin, hero::HeroPlugin, background::BackgroundPlugin, enemy::EnemyPlugin, loss::LossPlugin))
        .insert_resource(enemy::EnemyCount(7))
        .add_event::<LossEvent>()
        .add_systems(Update, quit_on_escape)
        .run();
}


fn quit_on_escape(keyboard_input: ResMut<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}