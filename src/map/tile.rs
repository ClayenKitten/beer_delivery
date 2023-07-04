use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// Every tile class is located in separate layer and has separate spritesheet.
pub trait TileClass: Clone + Component {
    fn texture(&self) -> u32;
}

/// A tile that doesn't allow player to go through it.
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct SolidTile {
    texture: u32,
}

impl TileClass for SolidTile {
    fn texture(&self) -> u32 {
        self.texture
    }
}

/// A tile that only exists for decoration purposes.
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct DecorationTile {
    pub texture: u32,
}

impl TileClass for DecorationTile {
    fn texture(&self) -> u32 {
        self.texture
    }
}

/// A tile that allows player to move between scenes or locations.
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct DoorTile {
    destination: String,
    texture: u32,
}

impl TileClass for DoorTile {
    fn texture(&self) -> u32 {
        self.texture
    }
}
