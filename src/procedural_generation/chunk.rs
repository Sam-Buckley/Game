use bevy::prelude::*;
use rand::{random, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// This defines a the BiomeType enum, which is used to define the type of biome that a chunk is.
#[derive(States, Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum BiomeType {
    Desert,
    Forest,
    Mountain,
    Ocean,
    Plains,
    Tundra,
    Derendered,
}

// This defines a the Chunk struct, which is used to define a chunk of the world. if has 40x40 tiles, and a biome type.
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Chunk {
    pub tiles: Vec<Vec<Tile>>,
    pub biome: BiomeType,
}

// This defines a the Tile struct, which is used to define a tile in the world. It has a type, and a height.
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
}

// This defines a the TileType enum, which is used to define the type of tile that a tile is.
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Grass,
    Sand,
    Snow,
    Stone,
    Water,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Position(pub i32, pub i32);

// Ruleset for adjacency
fn get_adjacency_rules() -> HashMap<TileType, Vec<TileType>> {
    let mut rules = HashMap::new();
    rules.insert(TileType::Grass, vec![TileType::Grass, TileType::Sand]);
    rules.insert(TileType::Sand, vec![TileType::Sand, TileType::Grass]);
    rules.insert(TileType::Snow, vec![TileType::Snow, TileType::Stone]);
    rules.insert(TileType::Stone, vec![TileType::Stone, TileType::Snow]);
    rules.insert(TileType::Water, vec![TileType::Water, TileType::Sand]); // Water can only be adjacent to sand - there will be rules about where sand can be placed
    rules
}

// Rules for biome generation - what biomes can be adjacent to each other
fn get_biome_rules() -> HashMap<BiomeType, Vec<BiomeType>> {
    let mut rules = HashMap::new();
    rules.insert(
        BiomeType::Desert,
        vec![BiomeType::Desert, BiomeType::Plains],
    );
    rules.insert(
        BiomeType::Forest,
        vec![BiomeType::Forest, BiomeType::Plains],
    );
    rules.insert(
        BiomeType::Mountain,
        vec![BiomeType::Mountain, BiomeType::Tundra],
    );
    rules.insert(BiomeType::Ocean, vec![BiomeType::Ocean]);
    rules.insert(
        BiomeType::Plains,
        vec![BiomeType::Plains, BiomeType::Desert, BiomeType::Forest],
    );
    rules.insert(
        BiomeType::Tundra,
        vec![BiomeType::Tundra, BiomeType::Mountain],
    );
    rules
}

fn generate_chunk(biome: BiomeType) -> Chunk {
    let mut rng = rand::thread_rng();
    let chunk_size = 40;
    let mut tiles = vec![
        vec![
            Tile {
                tile_type: TileType::Grass
            };
            chunk_size
        ];
        chunk_size
    ];

    for row in tiles.iter_mut() {
        for tile in row.iter_mut() {
            tile.tile_type = match biome {
                BiomeType::Desert => {
                    if random::<f32>() < 0.7 {
                        TileType::Sand
                    } else {
                        TileType::Stone
                    }
                }
                BiomeType::Forest => {
                    if random::<f32>() < 0.7 {
                        TileType::Grass
                    } else {
                        TileType::Stone
                    }
                }
                BiomeType::Mountain => {
                    if random::<f32>() < 0.7 {
                        TileType::Stone
                    } else {
                        TileType::Snow
                    }
                }
                BiomeType::Ocean => TileType::Water,
                BiomeType::Plains => {
                    let roll = rng.gen_range(0..3);
                    match roll {
                        0 => TileType::Grass,
                        1 => TileType::Grass,
                        _ => TileType::Stone,
                    }
                }
                BiomeType::Tundra => {
                    if rng.gen_bool(0.7) {
                        TileType::Snow
                    } else {
                        TileType::Stone
                    }
                }
                BiomeType::Derendered => TileType::Grass,
            };
        }
    }

    Chunk { tiles, biome }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        // Display every tile in the chunk with a a color based on the tile type in a 40x40 grid
        for row in self.tiles.iter() {
            for tile in row.iter() {
                let c = match tile.tile_type {
                    TileType::Grass => "🌱",
                    TileType::Sand => "🏖️",
                    TileType::Snow => "❄️",
                    TileType::Stone => "🪨",
                    TileType::Water => "🌊",
                };
                s.push_str(c);
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

pub fn test() {
    let chunk = generate_chunk(BiomeType::Plains);
    println!("{}", chunk);
}

// Function to generate 10x10 chunks of a biome using the adjacency rules and generate_chunk function
pub fn generate_map() -> Vec<Vec<Chunk>> {
    let mut rng = thread_rng();
    // Generate a 10x10 grid of chunks
    let mut chunks = vec![
        vec![
            Chunk {
                tiles: vec![
                    vec![
                        Tile {
                            tile_type: TileType::Sand
                        };
                        10 // The number of tiles in a row
                    ];
                    10 // The number of rows
                ],
                biome: BiomeType::Plains,
            };
            10 // The number of chunks in a row
        ];
        10 // The number of rows
    ];
    // Generate every chunk in the grid randomly
    for row in chunks.iter_mut() {
        for chunk in row.iter_mut() {
            chunk.biome = match rng.gen_range(0..6) {
                0 => BiomeType::Desert,
                1 => BiomeType::Forest,
                2 => BiomeType::Mountain,
                3 => BiomeType::Ocean,
                4 => BiomeType::Plains,
                _ => BiomeType::Tundra,
            };
            *chunk = generate_chunk(chunk.biome);
        }
    }
    chunks
}
