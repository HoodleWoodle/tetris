use crate::engine::{
    vec::Vec2f,
    graphics::{
        DrawParam, Rect,
        spritebatch::SpriteBatch,
    },
};
use std::cmp::PartialEq;

use crate::settings::Settings;
use crate::map::Map;

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
    pub fn from_usize(n: usize) -> TileType {
        match n {
            0 => TileType::I,
            1 => TileType::O,
            2 => TileType::T,
            3 => TileType::S,
            4 => TileType::Z,
            5 => TileType::J,
            6 => TileType::L,
            _ => TileType::Empty,
       }
    }

    pub fn draw_map(self, settings: &Settings, batch: &mut SpriteBatch, level: usize, map_position: &Vec2f, pos: Vec2f) {
        if pos.y < 2.0 {
            return;
        }

        let x = map_position.x + pos.x * settings.tile.size;
        let y = map_position.y + (pos.y - 2.0) * settings.tile.size;

        self.draw(settings, batch, level, Vec2f::new(x, y));
    }

    fn draw(self, settings: &Settings, batch: &mut SpriteBatch, level: usize, pos: Vec2f) {
        let rect = Rect::new(((self as i32) as f32) * 0.125, ((level % 10) as f32) * 0.1, 0.125, 0.1);
        let size = Vec2f::new(settings.tile.size, settings.tile.size);
        let draw_param = DrawParam::default()
            .src(rect)
            .size(Some(size))
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
    pub pos: Vec2f,
    orientation: Orientation,
    pub tiles: [Vec2f; 4],
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
            pos: Vec2f::new(4.5, 1.5),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new( 1.5, -0.5),
                Vec2f::new( 0.5, -0.5),
                Vec2f::new(-0.5, -0.5),
                Vec2f::new(-1.5, -0.5),
            ],
        }
    }

    fn new_o() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::O,
            pos: Vec2f::new(4.5, 0.5),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new(-0.5, -0.5),
                Vec2f::new(-0.5,  0.5),
                Vec2f::new( 0.5, -0.5),
                Vec2f::new( 0.5,  0.5),
            ],
        }
    }

    fn new_t() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::T,
            pos: Vec2f::new(4.0, 1.0),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new(-1.0,  0.0),
                Vec2f::new( 1.0,  0.0),
                Vec2f::new( 0.0,  0.0),
                Vec2f::new( 0.0, -1.0),
            ],
        }
    }

    fn new_s() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::S,
            pos: Vec2f::new(4.0, 1.0),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new(-1.0,  0.0),
                Vec2f::new( 0.0,  0.0),
                Vec2f::new( 0.0, -1.0),
                Vec2f::new( 1.0, -1.0),
            ],
        }
    }

    fn new_z() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::Z,
            pos: Vec2f::new(4.0, 1.0),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new(-1.0, -1.0),
                Vec2f::new( 0.0,  0.0),
                Vec2f::new( 0.0, -1.0),
                Vec2f::new( 1.0,  0.0),
            ],
        }
    }

    fn new_j() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::J,
            pos: Vec2f::new(4.0, 1.0),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new( 0.0,  0.0),
                Vec2f::new(-1.0,  0.0),
                Vec2f::new( 1.0,  0.0),
                Vec2f::new(-1.0, -1.0),
            ],
        }
    }

    fn new_l() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::L,
            pos: Vec2f::new(4.0, 1.0),
            orientation: Orientation::Deg0,
            tiles: [
                Vec2f::new( 0.0,  0.0),
                Vec2f::new(-1.0,  0.0),
                Vec2f::new( 1.0,  0.0),
                Vec2f::new( 1.0, -1.0),
            ],
        }
    }

    pub fn mov(&mut self, map: &Map, x_off: f32, y_off: f32) -> bool {
        self.pos.x += x_off;
        self.pos.y += y_off;

        if map.collision(self) {
            self.pos.x -= x_off;
            self.pos.y -= y_off;
            return false;
        }

        true
    }

    pub fn rotate(&mut self, wall_kicks_enabled: bool, map: &Map, right: bool) -> bool {
        if self.tile_type == TileType::O {
            return true;
        }

        let mut new_tet = Tetrimino {
            tile_type: self.tile_type,
            pos: self.pos,
            orientation: self.orientation.rotate(right),
            tiles: [Vec2f::new(0.0, 0.0); 4],
        };

        for i in 0..4 {
            // clockwise rotation
            // (1,0) -> (0,1)
            // (0,1) -> (-1,0)
            // A x = x'
            // A = [ [0,-1], [1,0]]

            let mut x = self.tiles[i].y;
            let mut y = -self.tiles[i].x;

            if right {
                x = -x;
                y = -y;
            }

            new_tet.tiles[i] = Vec2f::new(x, y);
        }

        if map.collision(&new_tet) {
            // wall kicks
            if wall_kicks_enabled {
                let rotation_direction_index = if right { 0 } else { 1 };
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

    pub fn draw_map(&self, settings: &Settings, batch: &mut SpriteBatch, level: usize, map_position: &Vec2f) {
        for &pos in self.tiles.iter() {
            let final_pos = Vec2f::new(self.pos.x + pos.x, self.pos.y + pos.y);
            self.tile_type.draw_map(settings, batch, level, map_position, final_pos);
        }
    }

    pub fn draw(&self, settings: &Settings, batch: &mut SpriteBatch, level: usize, offset: Vec2f) {
        for &pos in self.tiles.iter() {
            let x = offset.x + (pos.x - 0.5) * settings.tile.size;
            let y = offset.y + (pos.y - 0.5) * settings.tile.size;
            let final_pos = Vec2f::new(x, y);
            self.tile_type.draw(settings, batch, level, final_pos);
        }
    }
}
