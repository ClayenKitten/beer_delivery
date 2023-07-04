//! [MapData] is a data object used to save and load tilemaps to and from the disk.

pub mod preset;
pub mod tile;

use std::{ops::IndexMut, path::Path, slice::ChunksExact};

use bevy::prelude::Component;
use bevy_ecs_tilemap::prelude::TilemapSize;
use rmp_serde::{decode, encode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use tile::TileClass;

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct MapData<Tile: TileClass> {
    spritesheet: String,
    tiles: Vec<Option<Tile>>,
    width: usize,
}

impl<T: TileClass> MapData<T> {
    pub fn new(width: usize, height: usize, spritesheet: String) -> Self {
        Self {
            spritesheet,
            tiles: vec![None; width * height],
            width,
        }
    }

    pub fn iter(&self) -> MapIterator<'_, T> {
        MapIterator::new(self)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Option<T> {
        self.tiles.index_mut(x + y * self.width)
    }

    pub fn size(&self) -> TilemapSize {
        TilemapSize {
            x: self.width as u32,
            y: (self.tiles.len() / self.width) as u32,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MapLoadError {
    #[error("couldn't load map file: {0}")]
    FileSystemError(#[from] std::io::Error),
    #[error("provided file is not in valid format: {0}")]
    DecodeError(#[from] rmp_serde::decode::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum MapSaveError {
    #[error("couldn't save map to file: {0}")]
    FileSystemError(#[from] std::io::Error),
    #[error("provided map could not be encoded: {0}")]
    EncodeError(#[from] rmp_serde::encode::Error),
}

impl<T> MapData<T>
where
    T: TileClass + DeserializeOwned,
{
    pub fn load(path: &Path) -> Result<Self, MapLoadError> {
        let bytes = std::fs::read(path)?;
        let map = decode::from_slice::<MapData<T>>(&bytes)?;
        Ok(map)
    }
}

impl<T> MapData<T>
where
    T: TileClass + Serialize,
{
    pub fn save(&self, path: &Path) -> Result<(), MapSaveError> {
        let bytes = encode::to_vec(self)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

pub struct MapIterator<'map, Tile: TileClass> {
    chunks: ChunksExact<'map, Option<Tile>>,
}

impl<'map, Tile> MapIterator<'map, Tile>
where
    Tile: TileClass,
{
    pub fn new(map: &'map MapData<Tile>) -> Self {
        let row_size = map.width;
        Self {
            chunks: map.tiles.chunks_exact(row_size),
        }
    }
}

impl<'map, Tile> Iterator for MapIterator<'map, Tile>
where
    Tile: TileClass,
{
    type Item = &'map [Option<Tile>];

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next()
    }
}
