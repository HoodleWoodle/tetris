use crate::ggwp::{
    mint::Point2,
    graphics::spritebatch::SpriteBatch,
};
use std::ops::Index;

use crate::tetrimino::{TileType, Tetrimino};
use crate::settings::{self, Settings};

pub struct CompleteLines {
    data: Vec<usize>,
}

impl CompleteLines {
    pub fn new() -> CompleteLines {
        CompleteLines {
            data: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    fn push(&mut self, line: usize) {
        self.data.push(line);
    }
}

impl Index<usize> for CompleteLines {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

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

    pub fn complete_lines(&self) -> CompleteLines {
        let mut lines = CompleteLines::new();

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

    pub fn clear(&mut self, lines: &CompleteLines) {
        // remove complete lines
        for i in 0..lines.len() {
            for y in (lines[i + 1]..lines[i]).rev() {
                for x in 0..settings::MAP_WIDTH {
                    let tile_type = self.get(x, y);
                    self.set(x, y + i + 1, tile_type);
                }
            }
        }

        //
        for i in 0..lines.len() {
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
    
    pub fn draw(&self, settings: &Settings, batch: &mut SpriteBatch, level: usize, map_position: &Point2<f32>) {
        for y in 0..settings::MAP_HEIGHT {
            for x in 0..settings::MAP_WIDTH {
                let pos = Point2::new(x as f32, y as f32);
                self.tiles[y * settings::MAP_WIDTH + x].draw_map(settings, batch, level, map_position, pos);
            }
        }
    }

    pub fn reset(&mut self) {
        self.tiles = [TileType::Empty; settings::MAP_TILE_COUNT];
    }
}
