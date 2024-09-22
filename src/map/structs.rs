// === Structures ===

// === Imports ===
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// === Map ===
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
}

// === Tile ===
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

// === TileType ===
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TileType {
    #[default]
    Grass,
    Water,
    Sand,
    Stone,
    Tree,
    Bush,
    Rock,
    Dirt,
    Mountain,
    Snow,
    Ice,
    Lava,
    Void,
}

// === Chunk ===
#[derive(Debug, Clone, Resource, Default, Serialize, Deserialize)]
pub struct Chunk {
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
    pub biome: Biome,
}

// === Biome ===
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Biome {
    #[default]
    Grassland,
    Desert,
    Tundra,
    Taiga,
    Rainforest,
    Savanna,
    Alpine,
    Wetland,
    Swamp,
    Ocean,
    Beach,
    Volcanic,
    Void,
}
