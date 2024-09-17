use super::hero::Hero;
use super::loss::LossEvent;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
        app.add_systems(Update, enemy_movement.run_if(ready_timer_rc));
        app.add_systems(Update, enemy_bounds);
        app.add_systems(Update, collision_detection);
    }
}

#[derive(Resource)]
pub struct EnemyCount(pub u32);


#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
    veclocity: f32,
}

fn spawn_enemies(
    enemy_count: Res<EnemyCount>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..enemy_count.0 {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();
        let mut transform = Transform::from_xyz(x, y, 0.0);
        transform.scale = Vec3::splat(0.3);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/ball.png"),
                transform,
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0),
                veclocity: 500.0,
            },
        ));
    }
}

fn enemy_movement(
    delta: Res<Time>,
    mut query: Query<(&mut Transform, &Enemy)>,
) {
    for (mut transform, enemy) in query.iter_mut() {
        transform.translation += enemy.direction.extend(0.0) * enemy.veclocity * delta.delta_seconds();
    }
}

fn enemy_bounds(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Enemy)>,
) {
    let window = window_query.get_single().unwrap();
    for (mut transform, mut enemy) in query.iter_mut() {
        if transform.translation.x < 0.0 {
            transform.translation.x = 0.0;
            enemy.direction.x *= -1.0;
            // Rotate the enemy slightly
        }
        if transform.translation.x > window.width() {
            transform.translation.x = window.width();
            enemy.direction.x *= -1.0;
            // Rotate the enemy slightly
        }
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            enemy.direction.y *= -1.0;
            // Rotate the enemy slightly
        }
        if transform.translation.y > window.height() {
            transform.translation.y = window.height();
            enemy.direction.y *= -1.0;
            // Rotate the enemy slightly
        }
    }
}

fn collision_detection(
    mut commands: Commands,
    mut hero_query: Query<(Entity, &Transform), With<Hero>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut loss_event: EventWriter<LossEvent>,
) {
    for (hero_entity, hero_transform) in hero_query.iter_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = hero_transform.translation.distance(enemy_transform.translation);
            if distance < 50.0 {
                commands.entity(hero_entity).despawn();
                loss_event.send(LossEvent(true));
            }
        }
    }
}

fn ready_timer_rc(
    time: Res<Time>,
) -> bool {
    if time.elapsed_seconds() > 1.5 {
        true
    } else {
        false
    }
}