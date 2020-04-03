use ggez::{
    mint::Point2,
    graphics::{
        Color,
        spritebatch::SpriteBatch,
    },
};

use crate::tetrimino::{TileType, Tetrimino};
use crate::settings::{self, Settings, Point};

#[derive(Clone)]
pub struct Map {
    tiles: [TileType; settings::MAP_TILE_COUNT],
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: [TileType::Empty; settings::MAP_TILE_COUNT],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> TileType {
        self.tiles[settings::MAP_WIDTH * y + x]
    }

    pub fn set(&mut self, x: usize, y: usize, tile_type: TileType) {
        self.tiles[settings::MAP_WIDTH * y + x] = tile_type;
    }

    pub fn collision(&self, tet: &Tetrimino) -> bool {
        for &tile in tet.tiles.iter() {
            let x = (tet.pos.x + tile.x).round() as usize;
            let y = (tet.pos.y + tile.y).round() as usize;

            // (x < 0 || y < 0) is tested within next check because of usize wrap-around

            if x >= settings::MAP_WIDTH || y >= settings::MAP_HEIGHT {
                return true;
            }

            if self.tiles[y * settings::MAP_WIDTH + x] != TileType::Empty {
                return true;
            }
        }

        false
    }

    pub fn complete_lines(&mut self) -> Vec<usize> {
        let mut lines = Vec::new();

        for y in (0..settings::MAP_HEIGHT).rev() {
            let mut complete = true;

            for x in 0..settings::MAP_WIDTH {
                if self.get(x, y) == TileType::Empty {
                    complete = false;
                    break;
                }
            }

            if complete {
                lines.push(y);
            }
        }

        if !lines.is_empty() {
            lines.push(0);
        }

        lines
    }

    pub fn clear(&mut self, lines: &Vec<usize>) {
        let count = lines.len() - 1;

        // remove complete lines
        for i in 0..count {
            for y in (lines[i + 1]..lines[i]).rev() {
                for x in 0..settings::MAP_WIDTH {
                    let tile_type = self.get(x, y);
                    self.set(x, y + i + 1, tile_type);
                }
            }
        }

        //
        for i in 0..count {
            for x in 0..settings::MAP_WIDTH {
                self.set(x, i, TileType::Empty);
            }
        }
    }

    pub fn apply(&mut self, tet: &Tetrimino) {
        for &pos in tet.tiles.iter() {
            let x = (tet.pos.x + pos.x).round() as usize;
            let y = (tet.pos.y + pos.y).round() as usize;

            self.set(x, y, tet.tile_type);
        }
    }
    
    pub fn draw(&self, settings: &Settings, batch: &mut SpriteBatch, color: Color, level: usize, map_position: &Point) {
        for y in 0..settings::MAP_HEIGHT {
            for x in 0..settings::MAP_WIDTH {
                let pos: Point2<f32>  = Point2 { x: x as f32, y: y as f32 };
                self.tiles[y * settings::MAP_WIDTH + x].draw_map(settings, batch, color, level, map_position, pos);
            }
        }
    }

    pub fn reset(&mut self) {
        self.tiles = [TileType::Empty; settings::MAP_TILE_COUNT];
    }
}
