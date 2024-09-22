// === Plan ===
// We will be making 40x40 chunks of 40x40px tiles
// we will compile them into a single texture and write them to a file for use in the game

// === Imports ===

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod mapbuilder;

pub mod structs;

pub mod systems;

// === Plugin ===

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system();
    }
}
