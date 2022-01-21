#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Object {
    Pine,
    Oak,
    LittlePine,
    LittleOak,
    Stump,
    BigHouse,
    Worker
}

impl Object {
    pub fn blit(&self, x: i32, y: i32) {
        match self {
            Object::Pine | Object::Oak => {
                let start_tile_x = match self {
                    Object::Pine => 5,
                    _ => 7
                };
                let start_x = x - 8; // we shift our tree by one whole tile to the left
                let start_y = y - 16; // and by two whole tiles to the up
                for j in 0..3 {
                    for i in 0..2 {
                        let tile = &crate::sprites::TILES[j * 16 + i + start_tile_x];
                        tile.blit_as_sprite(i as i32 * 8 + start_x , j as i32 * 8 + start_y);
                    }
                }
            },
            Object::LittlePine => {
                let tile = &crate::sprites::TILES[52];
                tile.blit_as_sprite(x - 4, y - 8);
            },
            Object::LittleOak => {
                let tile = &crate::sprites::TILES[53];
                tile.blit_as_sprite(x - 4, y - 8);
            },
            Object::Stump => {
                let tile = &crate::sprites::TILES[53];
                tile.blit_as_sprite(x - 4, y - 4);
            },
            Object::BigHouse => {
                let start_x = x - 8; // we shift our house by one whole tile to the left
                let start_y = y - 8; // and by one whole tile to the up
                let start_tile_x = 9;
                for j in 0..4 {
                    for i in 0..3 {
                        let tile = &crate::sprites::TILES[j * 16 + i + start_tile_x];
                        tile.blit_as_sprite(i as i32 * 8 + start_x , j as i32 * 8 + start_y);
                    }
                }
            },
            Object::Worker => {
                crate::sprites::TILES[14].blit_as_sprite(
                    x - 1,
                    y - 7
                );
            },
            _ => ()
        }
    }
}

const WANG_INDICES_LOOKUP: [usize; 15] = [
    // we ignore completely zero bits since it's just a background color
    20, // NORTH_WEST_BITS
    19, // NORTH_EAST_BITS
    1, // NORTH_WEST_BITS | NORTH_EAST_BITS
    4, // SOUTH_WEST_BITS
    16, // NORTH_WEST_BITS | SOUTH_WEST_BITS
    36, // NORTH_EAST_BITS | SOUTH_WEST_BITS
    0, // NORTH_WEST_BITS | NORTH_EAST_BITS | SOUTH_WEST_BITS
    3, // SOUTH_EAST_BITS
    35, // NORTH_WEST_BITS | SOUTH_EAST_BITS
    18, // NORTH_EAST_BITS | SOUTH_EAST_BITS
    2, // NORTH_WEST_BITS | NORTH_EAST_BITS | SOUTH_EAST_BITS
    33, // SOUTH_WEST_BITS | SOUTH_EAST_BITS
    32, // NORTH_WEST_BITS | SOUTH_WEST_BITS | SOUTH_EAST_BITS
    34, // NORTH_EAST_BITS | SOUTH_WEST_BITS | SOUTH_EAST_BITS
    48 // ALL BITS SET
];

pub struct TerrainWangTile { pub bits: u8 }
impl TerrainWangTile {
    pub fn blit(&self, x: i32, y: i32) {
        if self.bits == 0 { return; }
        let tile = &super::sprites::TILES[WANG_INDICES_LOOKUP[self.bits as usize - 1]];
        tile.blit_as_tile(x, y);
    }
}

