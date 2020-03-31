use ggez::{
    mint::Point2,
    graphics::{
        DrawParam, Rect,
        spritebatch::SpriteBatch,
    },
};
use std::cmp::PartialEq;

use crate::settings::{self, Settings, Point};

const WALL_KICK_DATA_TSZJL: [[[(f32, f32); 5]; 4]; 2] = [
    [
        // Deg0 >> Deg90
        [( 0.0, 0.0), (-1.0, 0.0), (-1.0, 1.0), ( 0.0,-2.0), (-1.0,-2.0)],
        // Deg90 >> Deg180
        [( 0.0, 0.0), ( 1.0, 0.0), ( 1.0,-1.0), ( 0.0, 2.0), ( 1.0, 2.0)],
        // Deg180 >> Deg270
        [( 0.0, 0.0), ( 1.0, 0.0), ( 1.0, 1.0), ( 0.0,-2.0), ( 1.0,-2.0)],
        // Deg270 >> Deg0
        [( 0.0, 0.0), (-1.0, 0.0), (-1.0,-1.0), ( 0.0, 2.0), (-1.0, 2.0)],
    ],
    [
        // Deg90 >> Deg0
        [(0.0, 0.0), ( 1.0, 0.0), ( 1.0,-1.0), ( 0.0, 2.0), ( 1.0, 2.0)],
        // Deg180 >> Deg90
        [(0.0, 0.0), (-1.0, 0.0), (-1.0, 1.0), ( 0.0,-2.0), (-1.0,-2.0)],
        // Deg270 >> Deg180
        [(0.0, 0.0), (-1.0, 0.0), (-1.0,-1.0), ( 0.0, 2.0), (-1.0, 2.0)],
        // Deg0 >> Deg270
        [(0.0, 0.0), ( 1.0, 0.0), ( 1.0, 1.0), ( 0.0,-2.0), ( 1.0,-2.0)],
    ],
];

const WALL_KICK_DATA_I: [[[(f32, f32); 5]; 4]; 2] = [
    [
        // Deg0 >> Deg90
        [( 0.0, 0.0), (-2.0, 0.0), ( 1.0, 0.0), (-2.0,-1.0), ( 1.0, 2.0)],
        // Deg90 >> Deg180
        [( 0.0, 0.0), (-1.0, 0.0), ( 2.0, 0.0), (-1.0, 2.0), ( 2.0,-1.0)],
        // Deg180 >> Deg270
        [( 0.0, 0.0), ( 2.0, 0.0), (-1.0, 0.0), ( 2.0, 1.0), (-1.0,-2.0)],
        // Deg270 >> Deg0
        [( 0.0, 0.0), ( 1.0, 0.0), (-2.0, 0.0), ( 1.0,-2.0), (-2.0, 1.0)],
    ],
    [
        // Deg90 >> Deg0
        [( 0.0, 0.0), ( 2.0, 0.0), (-1.0, 0.0), ( 2.0, 1.0), (-1.0,-2.0)],
        // Deg180 >> Deg90
        [( 0.0, 0.0), ( 1.0, 0.0), (-2.0, 0.0), ( 1.0,-2.0), (-2.0, 1.0)],
        // Deg270 >> Deg180
        [( 0.0, 0.0), (-2.0, 0.0), ( 1.0, 0.0), (-2.0,-1.0), ( 1.0, 2.0)],
        // Deg0 >> Deg270
        [( 0.0, 0.0), (-1.0, 0.0), ( 2.0, 0.0), (-1.0, 2.0), ( 2.0,-1.0)],
    ],
];

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    Empty,
}

impl TileType {
    pub fn draw_map(self, settings: &Settings, batch: &mut SpriteBatch, level: usize, map_position: &Point, pos: Point2<f32>) {
        if pos.y < 2.0 {
            return;
        }

        let x = map_position.x + pos.x * settings.tile.size;
        let y = map_position.y + (pos.y - 2.0) * settings.tile.size;

        self.draw(batch, level, Point2 { x, y });
    }

    fn draw(self, batch: &mut SpriteBatch, level: usize, pos: Point2<f32>) {
        let rect = Rect::new(((self as i32) as f32) * 0.125, ((level % 10) as f32) * 0.1, 0.125, 0.1);
        let draw_param = DrawParam::default()
            .src(rect)
            .dest(pos);

        batch.add(draw_param);
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Orientation {
    fn rotate(&self, clockwise: bool) -> Orientation {
        match self {
            Orientation::Deg0 => if clockwise { Orientation::Deg90 } else { Orientation::Deg270 },
            Orientation::Deg90 => if clockwise { Orientation::Deg180 } else { Orientation::Deg0 },
            Orientation::Deg180 => if clockwise { Orientation::Deg270 } else { Orientation::Deg90 },
            Orientation::Deg270 => if clockwise { Orientation::Deg0 } else { Orientation::Deg180 },
        }
    }
}

#[derive(Clone)]
pub struct Tetrimino {
    pub tile_type: TileType,
    pub pos: Point2<f32>,
    orientation: Orientation,
    pub tiles: [Point2<f32>; 4],
}

impl Tetrimino {
    pub fn new(tile_type: TileType) -> Tetrimino {
        match tile_type {
            TileType::I => Tetrimino::new_i(),
            TileType::O => Tetrimino::new_o(),
            TileType::T => Tetrimino::new_t(),
            TileType::S => Tetrimino::new_s(),
            TileType::Z => Tetrimino::new_z(),
            TileType::J => Tetrimino::new_j(),
            TileType::L => Tetrimino::new_l(),
            _ => panic!("dead code"),
        }
    }

    fn new_i() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::I,
            pos: Point2 { x: 4.5, y: 1.5 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x:  1.5, y: -0.5 },
                Point2 { x:  0.5, y: -0.5 },
                Point2 { x: -0.5, y: -0.5 },
                Point2 { x: -1.5, y: -0.5 },
            ],
        }
    }

    fn new_o() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::O,
            pos: Point2 { x: 4.5, y: 0.5 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -0.5, y: -0.5 },
                Point2 { x: -0.5, y:  0.5 },
                Point2 { x:  0.5, y: -0.5 },
                Point2 { x:  0.5, y:  0.5 },
            ],
        }
    }

    fn new_t() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::T,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  1.0, y:  0.0 },
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x:  0.0, y: -1.0 },
            ],
        }
    }

    fn new_s() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::S,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x:  0.0, y: -1.0 },
                Point2 { x:  1.0, y: -1.0 },
            ],
        }
    }

    fn new_z() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::Z,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -1.0, y: -1.0 },
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x:  0.0, y: -1.0 },
                Point2 { x:  1.0, y:  0.0 },
            ],
        }
    }

    fn new_j() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::J,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  1.0, y:  0.0 },
                Point2 { x: -1.0, y: -1.0 },
            ],
        }
    }

    fn new_l() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::L,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  1.0, y:  0.0 },
                Point2 { x:  1.0, y: -1.0 },
            ],
        }
    }

    pub fn mov(&mut self, map: &[TileType; settings::MAP_TILE_COUNT], x_off: f32, y_off: f32) -> bool {
        self.pos.x += x_off;
        self.pos.y += y_off;

        if self.collision(map)
        {
            self.pos.x -= x_off;
            self.pos.y -= y_off;
            return false;
        }

        true
    }

    pub fn rotate(&mut self, settings: &Settings, map: &[TileType; settings::MAP_TILE_COUNT], clockwise: bool) -> bool {
        if self.tile_type == TileType::O {
            return true;
        }

        let mut new_tet = Tetrimino {
            tile_type: self.tile_type,
            pos: self.pos,
            orientation: self.orientation.rotate(clockwise),
            tiles: [Point2 { x: 0.0, y: 0.0 }; 4],
        };

        for i in 0..4 {
            // clockwise rotation
            // (1,0) -> (0,1)
            // (0,1) -> (-1,0)
            // A x = x'
            // A = [ [0,-1], [1,0]]

            let mut x = self.tiles[i].y;
            let mut y = -self.tiles[i].x;

            if clockwise {
                x = -x;
                y = -y;
            }

            new_tet.tiles[i] = Point2 { x, y };
        }

        if new_tet.collision(map) {
            // wall kicks
            if settings.wall_kicks_enabled {
                let rotation_direction_index = if clockwise { 0 } else { 1 };
                let orientation_index = self.orientation as usize;

                let data = if self.tile_type == TileType::I {
                    &WALL_KICK_DATA_I[rotation_direction_index][orientation_index]
                } else {
                    &WALL_KICK_DATA_TSZJL[rotation_direction_index][orientation_index]
                };

                for i in 0..5 {
                    if new_tet.mov(map, data[i].0, data[i].1) {
                        *self = new_tet;
                        return true;
                    }
                }
            }

            return false;
        }

        *self = new_tet;
        true
    }

    pub fn collision(&self, map: &[TileType; settings::MAP_TILE_COUNT]) -> bool {
        for &tile in self.tiles.iter() {
            let x = (self.pos.x + tile.x).round() as usize;
            let y = (self.pos.y + tile.y).round() as usize;

            // (x < 0 || y < 0) is tested within next check because of usize wrap-around

            if x >= settings::MAP_WIDTH || y >= settings::MAP_HEIGHT {
                return true;
            }

            if map[y * settings::MAP_WIDTH + x] != TileType::Empty {
                return true;
            }
        }

        false
    }

    pub fn draw_map(&self, settings: &Settings, batch: &mut SpriteBatch, level: usize, map_position: &Point) {
        for &pos in self.tiles.iter() {
            let final_pos = Point2 { x: (self.pos.x + pos.x), y: (self.pos.y + pos.y) };
            self.tile_type.draw_map(settings, batch, level, map_position, final_pos);
        }
    }

    pub fn draw(&self, settings: &Settings, batch: &mut SpriteBatch, level: usize, offset: Point2<f32>) {
        for &pos in self.tiles.iter() {
            let final_pos = Point2 { x: (offset.x + (pos.x - 0.5) * settings.tile.size), y: (offset.y + (pos.y - 0.5) * settings.tile.size) };
            self.tile_type.draw(batch, level, final_pos);
        }
    }
}