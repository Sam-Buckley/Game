use bevy::prelude::*;

pub mod chunk;
pub mod map;
pub mod systems;

pub struct ProceduralGenerationPlugin;

impl Plugin for ProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, chunk::test);
        app.add_plugins(map::MapPlugin);
    }
}
