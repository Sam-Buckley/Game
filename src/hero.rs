use bevy::window::PrimaryWindow;

use bevy::prelude::*;

use crate::loss;

const HERO_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Hero {
    direction: Vec2,
    veclocity: f32,
}

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hero);
        app.add_systems(Update, hero_direction.run_if(holding_wasd));
        app.add_systems(Update, hero_bounds);
        app.add_systems(Update, hero_movement);
        app.add_systems(Update, stop_movement.run_if(tapped_space));
    }
}

fn setup_hero(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0);
    transform.scale = Vec3::splat(4.0);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Hero.png"),
            transform,
            ..Default::default()
        },
        Hero {
            direction: Vec2::new(1.0, 0.0),
            veclocity: 500.0,
        },
    ));
}

fn hero_movement(
    delta: Res<Time>,
    mut query: Query<(&mut Transform, &Hero)>,
) {
    for (mut transform, hero) in query.iter_mut() {
        transform.translation += hero.direction.extend(0.0) * hero.veclocity * delta.delta_seconds();
    }
}

fn stop_movement(
    mut query: Query<&mut Hero>
) {
    for mut hero in query.iter_mut() {
        hero.direction = Vec2::ZERO;
    }
}

fn hero_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Hero, &mut Sprite)>,
) {
    for (mut hero, mut hero_sprite) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            // Flip the sprite when moving left
            hero_sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            // Don't flip the sprite when moving right
            hero_sprite.flip_x = false;
        }

        if direction != Vec2::ZERO {
            hero.direction = direction.normalize();
        }
    }
}

#[allow(dead_code, unused_variables, unused_mut)]
fn hero_bounds(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut hero_query: Query<&mut Transform, With<Hero>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let half_hero_size = HERO_SIZE / 2.0;
    let y_min = half_hero_size;
    let y_max = window_height - half_hero_size;
    let x_min = half_hero_size;
    let x_max = window_width - half_hero_size;

    if let Ok(mut hero_transform) = hero_query.get_single_mut() {
        if hero_transform.translation.y < y_min {
            hero_transform.translation.y = y_min;
        } else if hero_transform.translation.y > y_max {
            hero_transform.translation.y = y_max;
        }

        if hero_transform.translation.x < x_min {
            hero_transform.translation.x = x_min;
        } else if hero_transform.translation.x > x_max {
            hero_transform.translation.x = x_max;
        }
    }
}


fn holding_wasd(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::KeyS)
        || keyboard_input.pressed(KeyCode::KeyD)
}

fn tapped_space(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
}

fn reset (keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::KeyR)
}

fn restart_game(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Transform), With<Hero>>,
    mut loss_event: EventWriter<loss::LossEvent>,
) {
    if reset(keyboard_input) {
        for (entity, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
        // Reset the loss event
        loss_event.send(loss::LossEvent(false));
        // Reset the hero
    
    }
}