pub mod movement;
pub mod spawning;

use bevy::prelude::*;

use crate::main_menu::systems::layout::*;
use crate::states::AppState::{self, *};
use crate::DespawnedYet;

pub struct PlayerSpawnMovementPlugin;

impl Plugin for PlayerSpawnMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(InGame),
            spawning::spawn_player
                .after(despawn_main_menu)
                .run_if(not_despawned),
        );
        app.add_systems(
            OnExit(InGame),
            spawning::despawn_player.before(spawn_main_menu),
        );
        app.add_systems(
            OnEnter(InGame),
            spawning::respawn_player
                .run_if(despawned)
                .after(despawn_main_menu),
        );
        app.add_plugins(movement::PlayerMovementPlugin);
    }
}

fn despawned(state: Res<State<DespawnedYet>>) -> bool {
    *state.get() == DespawnedYet::Yes
}

fn not_despawned(state: Res<State<DespawnedYet>>) -> bool {
    *state.get() == DespawnedYet::No
}
