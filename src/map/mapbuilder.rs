// === Procedurally Generates Maps ===

// === Imports ===
use bevy::prelude::*;

use std::collections::HashMap;

use crate::map::structs::*;

// === Rules ===

// We will be making 40x40 chunks of 40x40px tiles
// Each chunk will have a biome - there are rules around neighbouring biomes and how they can be placed, as well as the frequency of each biome
// Each chunk will store a 2D array of tiles - each tile will have a type and a position, there will be rules around how these tiles can be placed
// We will compile the chunks into a single texture and write them to a file for use in the game

fn biome_adjacency() -> HashMap<Biome, Vec<Biome>> {
    let mut adjacency = HashMap::new();
    adjacency.insert(
        Biome::Grassland,
        vec![
            Biome::Desert,
            Biome::Tundra,
            Biome::Taiga,
            Biome::Rainforest,
            Biome::Savanna,
        ],
    );
    adjacency.insert(
        Biome::Desert,
        vec![
            Biome::Grassland,
            Biome::Tundra,
            Biome::Taiga,
            Biome::Rainforest,
            Biome::Savanna,
        ],
    );
    adjacency.insert(
        Biome::Tundra,
        vec![
            Biome::Grassland,
            Biome::Desert,
            Biome::Taiga,
            Biome::Rainforest,
            Biome::Savanna,
        ],
    );
    adjacency.insert(
        Biome::Taiga,
        vec![
            Biome::Grassland,
            Biome::Desert,
            Biome::Tundra,
            Biome::Rainforest,
            Biome::Savanna,
        ],
    );
    adjacency.insert(
        Biome::Rainforest,
        vec![
            Biome::Grassland,
            Biome::Desert,
            Biome::Tundra,
            Biome::Taiga,
            Biome::Savanna,
        ],
    );
    adjacency.insert(
        Biome::Savanna,
        vec![
            Biome::Grassland,
            Biome::Desert,
            Biome::Tundra,
            Biome::Taiga,
            Biome::Rainforest,
        ],
    );
    adjacency
}
