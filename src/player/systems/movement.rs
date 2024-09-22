use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::Camera;
use crate::player::components::*;
use crate::states::AppState::*;

use crate::procedural_generation::map::*;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system.run_if(in_state(InGame)));
    }
}

fn setup(mut _commands: Commands) {
    // Add setup logic here
}

fn movement_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &PlayerStats, &mut Sprite), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut movement: ResMut<last_player_position>,
) {
    let window = window_query.get_single().unwrap();
    #[allow(unused_assignments)]
    let mut player_translation = Vec3::ZERO;

    if let Ok((mut player_transform, player_stats, mut sprite)) = player_query.get_single_mut() {
        let player_speed = player_stats.speed;
        player_translation = player_transform.translation;

        if keyboard_input.pressed(KeyCode::KeyW) {
            player_translation.y += player_speed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            player_translation.y -= player_speed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            player_translation.x -= player_speed;
            // Turn the player to the left
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player_translation.x += player_speed;
            // Turn the player to the right
            sprite.flip_x = false;
        }
        // normalise player speed

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let (lower_x_bound, upper_x_bound, lower_y_bound, upper_y_bound) =
                get_bounds(&camera_transform, &window);
            // select a value that's 15% of the window size to be the buffer
            // for the camera to start moving
            let buffer = 0.15;
            let lower_x_bound = lower_x_bound + window.width() * buffer;
            let upper_x_bound = upper_x_bound - window.width() * buffer;
            let lower_y_bound = lower_y_bound + window.height() * buffer;
            let upper_y_bound = upper_y_bound - window.height() * buffer;
            let within_bounds = player_translation.x < lower_x_bound
                || player_translation.x > upper_x_bound
                || player_translation.y < lower_y_bound
                || player_translation.y > upper_y_bound;

            if within_bounds {
                camera_transform.translation.x +=
                    player_translation.x - player_transform.translation.x;
                camera_transform.translation.y +=
                    player_translation.y - player_transform.translation.y;
            }
        }

        player_transform.translation = player_translation;
        *movement = last_player_position {
            x: player_translation.x,
            y: player_translation.y,
        };
    }
}

fn get_bounds(camera: &Transform, window: &Window) -> (f32, f32, f32, f32) {
    let camera_x = camera.translation.x;
    let camera_y = camera.translation.y;
    let camera_width = window.width();
    let camera_height = window.height();
    (
        camera_x - camera_width / 2.0,
        camera_x + camera_width / 2.0,
        camera_y - camera_height / 2.0,
        camera_y + camera_height / 2.0,
    )
}
