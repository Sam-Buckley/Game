use bevy::prelude::*;
pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(systems::PlayerSpawnMovementPlugin);
        app.insert_resource(components::DespawnedPlayer::new());
    }
}
