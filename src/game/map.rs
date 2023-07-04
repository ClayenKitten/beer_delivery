use std::path::Path;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    map::{
        tile::{DecorationTile, SolidTile, TileClass},
        MapData,
    },
    GameState,
};

use super::OnGameScreen;

/// Plugin that stores tilemap logic and data.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(
            setup
                .in_base_set(CoreSet::PreUpdate)
                .in_schedule(OnEnter(GameState::Game)),
        )
        .add_system(
            spawn_tilemap::<DecorationTile, 0>
                .in_base_set(CoreSet::Update)
                .run_if(in_state(GameState::Game)),
        )
        .add_system(
            spawn_tilemap::<SolidTile, 12>
                .in_base_set(CoreSet::Update)
                .run_if(in_state(GameState::Game)),
        );
    }
}

fn setup(mut commands: Commands) {
    let map_data1 = MapData::<DecorationTile>::load(Path::new("decoration.beer_map"));
    let map_data2 = MapData::<SolidTile>::load(Path::new("solid.beer_map"));

    if let (Ok(map_data1), Ok(map_data2)) = (map_data1, map_data2) {
        commands.spawn((map_data1, OnGameScreen));
        commands.spawn((map_data2, OnGameScreen));
    } else {
        println!("Invalid map(s). Loading preset...");
        crate::map::preset::generate_preset_maps().unwrap();
        setup(commands);
    }
}

fn spawn_tilemap<M, const Z: usize>(
    query: Query<&MapData<M>, Added<OnGameScreen>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    M: TileClass,
{
    for map in query.iter() {
        let texture_handle: Handle<Image> = asset_server.load("city_tiles.png");
        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map.size());

        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(cell) = cell {
                    let tile_pos = TilePos {
                        x: x as u32,
                        y: y as u32,
                    };
                    let tile_entity = commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(cell.texture()),
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }

        let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
        let grid_size = tile_size.into();
        let map_type = TilemapType::default();

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size: map.size(),
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map.size(), &grid_size, &map_type, Z as f32),
            ..Default::default()
        });
    }
}
