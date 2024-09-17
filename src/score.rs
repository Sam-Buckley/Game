//This will make a score in the top left that increments every second


use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::hero::Hero;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Resource)]
struct StarSpawnTimer(Timer);

#[derive(Resource)]
struct CountTimer(Timer);

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct STimer(pub Timer);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, score_system.run_if(collided));
        app.insert_resource(StarSpawnTimer(Timer::from_seconds(rand::random::<f32>() * 4.0 + 2.0, TimerMode::Repeating)));
        app.add_systems(Update, spawn_stars);
        app.insert_resource(CountTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
    }
}

#[derive(Component)]
pub struct Star;

fn score_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    score_query: Query<Entity, With<ScoreText>>,
    player_query: Query<&Transform, With<Hero>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    assets: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<CountTimer>
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok(player_transform) = player_query.get_single() {
            for (id, transform) in star_query.iter() {
                if transform.translation.distance(player_transform.translation) < 100.0 {
                    commands.entity(id).despawn()
                }
            }
        }
        score.0 += 1;
        
        let window = window_query.get_single().unwrap();
        let digits = calculate_digits(score.0);

        // despawn old score text
        for entity in score_query.iter() {
            commands.entity(entity).despawn();
        }

        // spawn new score text
        // Render assets/numbers/numberN.png for each digit N
        let mut x = 40.0;
        for digit in digits {
            let mut transform = Transform::from_xyz(x, window.height() - 100.0, 2.0);
            transform.scale = Vec3::splat(8.0);
            let texture = assets.load(format!("Numbers/Number{}.png", digit));
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform,
                    ..Default::default()
                },
                ScoreText,
            ));
            x += 50.0;
        }
    }
}

fn calculate_digits(score: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut score = score;
    while score > 0 {
        digits.push(score % 10);
        score /= 10;
    }
    digits.reverse();
    digits
}

fn collided(
    player_query: Query<&Transform, With<Hero>>,
    mut obstacle_query: Query<&Transform, With<Star>>
) -> bool {
    if let Ok(player_transform) = player_query.get_single() {
        for  obstacle_transform in obstacle_query.iter_mut() {
            let distance = player_transform.translation.distance(obstacle_transform.translation);
            if distance < 100.0 {
                return true;
            }
        }
    }
    false
}

use rand;


fn setup(mut commands: Commands) {
    commands.insert_resource(StarSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

fn spawn_stars(
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<StarSpawnTimer>
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        // Spawn star logic here
        if let Ok(window) = window_query.get_single() {
            let x = rand::random::<f32>() * window.width();
            let y = rand::random::<f32>() * window.height();
            let mut transform = Transform::from_xyz(x, y, 0.0);
            transform.scale = Vec3::splat(0.4);
            commands.spawn((SpriteBundle {
                texture: asset_server.load("sprites/star.png"),
                transform,
                ..Default::default()
            }, Star));

            // Reset timer with a new random duration between 2 to 6 seconds
            let new_duration = rand::random::<f32>() * 4. + 2.;
            timer.0.set_duration(std::time::Duration::from_secs_f32(new_duration));
            timer.0.reset();
        }
    }
}