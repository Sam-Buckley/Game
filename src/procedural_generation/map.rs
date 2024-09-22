use crate::player::components::*;
use crate::procedural_generation::chunk::generate_map;
use crate::procedural_generation::chunk::*;
use crate::states::AppState::InGame;
use crate::Active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

pub const TILE_WIDTH: f32 = 60.0;
pub const TILE_HEIGHT: f32 = 60.0;
pub const CHUNK_AREA: f32 = 10.0 * 10.0 * TILE_WIDTH * TILE_HEIGHT;
pub const CHUNK_WIDTH: f32 = 7.0 * TILE_WIDTH;
pub const CHUNK_HEIGHT: f32 = 7.0 * TILE_HEIGHT;
pub const CHUNKS: i32 = 10;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_map)
            .insert_resource(DerenderedChunks {
                chunks: HashMap::new(),
                ids: Vec::new(),
            })
            .insert_resource(RenderedChunks {
                chunks: HashMap::new(),
                ids: Vec::new(),
            })
            .init_state::<ChunkLoading>()
            .insert_resource(NextState::<RenderedYet>::default())
            .add_systems(Startup, derender_chunks.after(render_map))
            .add_systems(
                OnEnter(ChunkLoading::Loading),
                chunk_loader.run_if(in_state(InGame)),
            )
            .add_systems(
                OnEnter(ChunkLoading::Loading),
                render_loaded.after(derender_chunks),
            )
            .add_systems(
                OnEnter(ChunkLoading::Loading),
                derender_unloaded.after(render_loaded),
            )
            .insert_resource(ChunkTimer::default())
            .add_systems(Update, chunk_loader_timer)
            .init_resource::<last_player_position>();
    }
}

#[derive(Resource)]
struct Map {
    tiles: Vec<Vec<Chunk>>,
}

fn render_map(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let map = Map {
        tiles: generate_map(),
    };

    let window = window.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let mut chunk_x_offset = 0.0;
    let mut chunk_y_offset = 0.0;

    let textures = load_textures(&assets);
    let mut id: u32 = 0;

    for chunk_row in map.tiles.iter() {
        for chunk in chunk_row.iter() {
            let mut tile_x_offset = 0.0;
            let mut tile_y_offset = 0.0;
            for tile_row in chunk.tiles.iter() {
                tile_x_offset = 0.0;
                for tile in tile_row.iter() {
                    let texture = textures.get(&tile.tile_type).unwrap();

                    commands.spawn((
                        SpriteBundle {
                            texture: texture.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    tile_x_offset + chunk_x_offset,
                                    tile_y_offset + chunk_y_offset,
                                    -3.0,
                                ),
                                ..Default::default()
                            }
                            .with_scale(Vec3::splat(1.5)),
                            ..Default::default()
                        },
                        Coord {
                            x: (tile_x_offset + chunk_x_offset) as i32,
                            y: (tile_y_offset + chunk_y_offset) as i32,
                        },
                        tile.tile_type.clone(),
                        ID(id),
                    ));

                    tile_x_offset += TILE_WIDTH;
                }
                tile_y_offset += TILE_HEIGHT;
            }
            chunk_x_offset += CHUNK_WIDTH;
        }
        chunk_y_offset += CHUNK_HEIGHT;
        chunk_x_offset = 0.0;
    }
}

fn load_textures(assets: &Res<AssetServer>) -> HashMap<TileType, Handle<Image>> {
    let mut textures = HashMap::new();
    textures.insert(TileType::Grass, assets.load("tiles/grass.png"));
    textures.insert(TileType::Sand, assets.load("tiles/sand.png"));
    textures.insert(TileType::Snow, assets.load("tiles/snow.png"));
    textures.insert(TileType::Stone, assets.load("tiles/stone.png"));
    textures.insert(TileType::Water, assets.load("tiles/water.png"));
    textures
}

#[derive(Resource)]
struct DerenderedChunks {
    chunks: HashMap<Coord, Chunk>,
    ids: Vec<ID>,
}

#[derive(Resource)]
struct RenderedChunks {
    chunks: HashMap<Coord, Chunk>,
    ids: Vec<ID>,
}

#[derive(Component, Hash, Eq, PartialEq, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn derender_chunks(
    mut commands: Commands,
    tiles: Query<(Entity, &Coord, &TileType, &ID)>,
    mut derendered_chunks: ResMut<DerenderedChunks>,
    mut state: ResMut<NextState<RenderedYet>>,
) {
    let mut chunks: HashMap<Coord, Chunk> = HashMap::new();
    let mut ids: Vec<ID> = Vec::new();
    for (entity, coord, tile_type, id) in tiles.iter() {
        let chunk_coord = Coord {
            x: coord.x / 400,
            y: coord.y / 400,
        };
        let chunk = chunks.entry(chunk_coord).or_insert(Chunk {
            tiles: vec![vec![Tile {
                tile_type: TileType::Grass,
            }]],
            biome: BiomeType::Desert,
        });
        if chunk.tiles.last().unwrap().len() < 10 {
            chunk.tiles.last_mut().unwrap().push(Tile {
                tile_type: tile_type.clone(),
            });
        } else {
            chunk.tiles.push(vec![Tile {
                tile_type: tile_type.clone(),
            }]);
        }
        commands.entity(entity).despawn();
        ids.push(id.clone());
    }
    state.set(RenderedYet::Yes);
    derendered_chunks.chunks = chunks;
    derendered_chunks.ids = ids;
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum RenderedYet {
    #[default]
    No,
    Yes,
}

fn chunk_loader(
    player_position: Query<&Transform, With<Player>>,
    mut rendered_chunks: ResMut<RenderedChunks>,
    mut derendered_chunks: ResMut<DerenderedChunks>,
) {
    if let Some(player_transform) = player_position.iter().next() {
        let player_x = player_transform.translation.x;
        let player_y = player_transform.translation.y;
        let player_chunk_x = (player_x / 400.0).round() as i32;
        let player_chunk_y = (player_y / 400.0).round() as i32;

        // Load chunks within a 2-chunk radius vertically and a 3-chunk radius horizontally
        for y in (player_chunk_y - 2..=player_chunk_y + 2).rev() {
            for x in player_chunk_x - 3..=player_chunk_x + 3 {
                let coord = Coord { x, y };
                if !rendered_chunks.chunks.contains_key(&coord) {
                    if let Some(chunk) = derendered_chunks.chunks.remove(&coord) {
                        rendered_chunks.chunks.insert(coord, chunk);
                    }
                }
            }
        }

        let mut to_derender = Vec::new();
        for (coord, chunk) in rendered_chunks.chunks.iter() {
            if (player_chunk_x - coord.x).abs() > 3 || (player_chunk_y - coord.y).abs() > 2 {
                to_derender.push(coord.clone());
                derendered_chunks
                    .chunks
                    .insert(coord.clone(), chunk.clone());
            }
        }
        for coord in to_derender {
            rendered_chunks.chunks.remove(&coord);
        }
    }
}

fn render_loaded(
    mut commands: Commands,
    rendered: Res<RenderedChunks>,
    assets: Res<AssetServer>,
    mut chunkloading: ResMut<NextState<ChunkLoading>>,
) {
    if !rendered.is_changed() {
        return;
    }
    let textures = load_textures(&assets);
    let mut id: u32 = 0;
    for (coord, chunk) in rendered.chunks.iter() {
        let mut tile_x_offset = 0.0;
        let mut tile_y_offset = 0.0;
        for tile_row in chunk.tiles.iter() {
            tile_x_offset = 0.0;
            for tile in tile_row.iter() {
                let texture = textures.get(&tile.tile_type).unwrap();

                commands.spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                tile_x_offset + coord.x as f32 * 400.0,
                                tile_y_offset + coord.y as f32 * 400.0,
                                -3.0,
                            ),
                            ..Default::default()
                        }
                        .with_scale(Vec3::splat(1.5)),
                        ..Default::default()
                    },
                    Coord {
                        x: (tile_x_offset + coord.x as f32 * 400.0) as i32,
                        y: (tile_y_offset + coord.y as f32 * 400.0) as i32,
                    },
                    tile.tile_type.clone(),
                    ID(id), // This assumes
                ));

                tile_x_offset += TILE_WIDTH;
            }
            tile_y_offset += TILE_HEIGHT;
        }
    }
    chunkloading.set(ChunkLoading::NotLoading);
}

fn derender_unloaded(
    mut commands: Commands,
    derendered_chunks: Res<DerenderedChunks>,
    tiles: Query<(Entity, &Coord), With<TileType>>,
    mut loading: ResMut<NextState<ChunkLoading>>,
) {
    for (entity, coord) in tiles.iter() {
        // Divide by 400 to get the chunk coord
        let new = Coord {
            x: coord.x / 400,
            y: coord.y / 400,
        };
        if derendered_chunks.chunks.contains_key(&new) {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChunkLoading {
    Loading,
    #[default]
    NotLoading,
}

#[derive(Resource)]
pub struct ChunkTimer(Timer);

impl Default for ChunkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn chunk_loader_timer(
    mut state: ResMut<NextState<ChunkLoading>>,
    current_state: Res<State<ChunkLoading>>,
    mut timer: ResMut<ChunkTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        // Swap the state
        match current_state.get() {
            ChunkLoading::NotLoading => state.set(ChunkLoading::Loading),
            ChunkLoading::Loading => state.set(ChunkLoading::NotLoading),
        }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ID(pub u32);

#[derive(Resource, Default)]
pub struct last_player_position {
    pub x: f32,
    pub y: f32,
}
