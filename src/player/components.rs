// Components for the player entity & weapon entity

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy)]
pub struct PlayerStats {
    pub health: f32,
    pub speed: f32,
}

// Make a weapon asset
#[derive(Asset, Reflect)]
pub struct Weapon {
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,
}

#[derive(Resource)]
pub struct DespawnedPlayer {
    pub stats: PlayerStats,
    pub transform: Transform,
}

impl DespawnedPlayer {
    pub fn new() -> Self {
        Self {
            stats: PlayerStats {
                health: 100.0,
                speed: 5.0,
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
    fn set_stats(&mut self, stats: PlayerStats) {
        self.stats = stats;
    }
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl Default for DespawnedPlayer {
    fn default() -> Self {
        Self::new()
    }
}
