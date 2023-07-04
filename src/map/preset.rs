use std::path::Path;

use super::tile::*;
use super::MapData;
use super::MapSaveError;

pub fn generate_preset_maps() -> Result<(), MapSaveError> {
    generate_decoration_map()?;
    generate_solid_map()?;
    Ok(())
}

fn generate_decoration_map() -> Result<(), MapSaveError> {
    let mut map = MapData::<DecorationTile>::new(16, 16, String::from("city_tiles.png"));
    for x in 0..16 {
        for y in 0..16 {
            *map.get_mut(x, y) = Some(DecorationTile { texture: 28 });
        }
    }
    for x in 0..8 {
        if x == 7 {
            *map.get_mut(x, 6) = Some(DecorationTile {
                texture: 16 * 27 + 6,
            });
            *map.get_mut(x, 5) = Some(DecorationTile {
                texture: 17 * 27 + 4,
            });
            *map.get_mut(x, 4) = Some(DecorationTile {
                texture: 17 * 27 + 6,
            });
        } else if x == 0 {
            *map.get_mut(x, 6) = Some(DecorationTile {
                texture: 16 * 27 + 5,
            });
            *map.get_mut(x, 5) = Some(DecorationTile {
                texture: 17 * 27 + 2,
            });
            *map.get_mut(x, 4) = Some(DecorationTile {
                texture: 17 * 27 + 5,
            });
        } else {
            *map.get_mut(x, 6) = Some(DecorationTile {
                texture: 15 * 27 + 1,
            });
            *map.get_mut(x, 5) = Some(DecorationTile {
                texture: 16 * 27 + 1,
            });
            *map.get_mut(x, 4) = Some(DecorationTile {
                texture: 17 * 27 + 1,
            });
        }
    }

    map.save(Path::new("decoration.beer_map"))
}

fn generate_solid_map() -> Result<(), MapSaveError> {
    let mut map = MapData::<DecorationTile>::new(16, 16, String::from("city_tiles.png"));

    struct Bounds {
        pub min_x: usize,
        pub min_y: usize,
        pub max_x: usize,
        pub max_y: usize,
    }

    impl Bounds {
        pub fn location(&self, x: usize, y: usize) -> Location {
            if x == self.min_x && y == self.min_y {
                return Location::BottomLeft;
            }
            if x == self.min_x && y == self.max_y {
                return Location::TopLeft;
            }
            if x == self.max_x && y == self.min_y {
                return Location::BottomRight;
            }
            if x == self.max_x && y == self.max_y {
                return Location::TopRight;
            }
            if x == self.min_x {
                return Location::Left;
            }
            if x == self.max_x {
                return Location::Right;
            }
            if y == self.min_y {
                return Location::Bottom;
            }
            if y == self.max_y {
                return Location::Top;
            }
            Location::Center
        }
    }

    pub enum Location {
        Center,
        Top,
        Bottom,
        Left,
        Right,
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    let b = Bounds {
        min_x: 5,
        min_y: 10,
        max_x: 10,
        max_y: 14,
    };
    for x in b.min_x..=b.max_x {
        for y in b.min_y..=b.max_y {
            let texture = {
                match b.location(x, y) {
                    Location::Center => 4 * 27 + 9,
                    Location::Top => 3 * 27 + 9,
                    Location::Bottom => 5 * 27 + 9,
                    Location::Left => 4 * 27 + 8,
                    Location::Right => 4 * 27 + 10,
                    Location::TopLeft => 3 * 27 + 8,
                    Location::TopRight => 3 * 27 + 10,
                    Location::BottomLeft => 5 * 27 + 8,
                    Location::BottomRight => 5 * 27 + 10,
                }
            };
            *map.get_mut(x, y) = Some(DecorationTile { texture });
        }
    }

    let b = Bounds {
        min_x: 5,
        min_y: 8,
        max_x: 10,
        max_y: 9,
    };
    for x in b.min_x..=b.max_x {
        for y in b.min_y..=b.max_y {
            let texture = match b.location(x, y) {
                Location::Top => 18,
                Location::TopLeft => 17,
                Location::TopRight => 19,
                Location::Bottom => 18 + 27 * 3,
                Location::BottomLeft => 17 + 27 * 3,
                Location::BottomRight => 19 + 27 * 3,
                _ => unreachable!(),
            };
            *map.get_mut(x, y) = Some(DecorationTile { texture });
        }
    }

    map.save(Path::new("solid.beer_map"))
}
