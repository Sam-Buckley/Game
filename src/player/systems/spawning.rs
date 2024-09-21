use bevy::prelude::*;
use bevy::render::camera;
use bevy::utils::tracing::field::debug;
use bevy::window::PrimaryWindow;

use crate::camera::Camera;

use crate::player::components::*;
use crate::DespawnedYet;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let mut transform =
        Transform::from_translation(Vec3::new(window_width / 2.0, window_height / 2.0, 0.0))
            .with_scale(Vec3::splat(5.0));
    transform.scale = Vec3::splat(5.0);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Hero.png"),
            transform,
            ..Default::default()
        },
        PlayerStats {
            health: 100.0,
            speed: 10.0,
        },
        Player,
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &PlayerStats), With<Player>>,
    mut player_resource: ResMut<DespawnedPlayer>,
    // mut despawned: ResMut<NextState<DespawnedYet>>,
) {
    for (entity, transform, stats) in query.iter() {
        // Save the player's health and transformm>()
        player_resource.stats = stats.clone();
        player_resource.transform = *transform;
        commands.entity(entity).despawn();
    }
}

pub fn respawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_resource: Res<DespawnedPlayer>,
) {
    let transform = player_resource.transform;
    let stats = player_resource.stats;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/Hero.png"),
            transform,
            ..Default::default()
        },
        stats,
        Player,
    ));
}
