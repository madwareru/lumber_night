const NORTH_WEST_BITS: u8 = 0b00_01;
const NORTH_EAST_BITS: u8 = 0b00_10;
const SOUTH_WEST_BITS: u8 = 0b01_00;
const SOUTH_EAST_BITS: u8 = 0b10_00;

#[derive(Clone)]
pub struct Tilemap {
    pub map: [u8; 21 * 21]
}
impl Tilemap {
    pub fn draw(&self) {
        for j in 0..20 {
            for i in 0..20 {
                let nw = self.map[j * 21 + i];
                let ne = self.map[j * 21 + i + 1];
                let sw = self.map[(j + 1) * 21 + i];
                let se = self.map[(j + 1) * 21 + i + 1];
                let mut bits = if nw == 1 { NORTH_WEST_BITS } else { 0 };
                if ne == 1 { bits |= NORTH_EAST_BITS; }
                if sw == 1 { bits |= SOUTH_WEST_BITS; }
                if se == 1 { bits |= SOUTH_EAST_BITS; }
                let tile = super::objects::TerrainWangTile { bits };
                let x = (i * 2) as i32;
                let y = (j * 2) as i32;
                tile.blit(x, y);
            }
        }
    }
}