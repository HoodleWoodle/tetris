use rand::{
    rngs::StdRng,
    RngCore, SeedableRng,
};
use serde::Deserialize;

use crate::tetrimino::TileType;

pub trait RandomGenerator {
    fn next(&mut self) -> TileType;
}

#[derive(Copy, Clone, Deserialize)]
pub enum RandomGeneratorType {
    RandomBag,
    RandomNES,
}

pub struct RandomBag {
    rng: StdRng,
    types: Vec<TileType>,
}

impl RandomBag {
    fn new(seed: [u8; 32]) -> RandomBag {
        RandomBag {
            rng: StdRng::from_seed(seed),
            types: Vec::with_capacity(7),
        }
    }
}

impl RandomGenerator for RandomBag {
    fn next(&mut self) -> TileType {
        let mut len = self.types.len();

        if len == 0 {
            self.types.extend_from_slice(&[
                TileType::I,
                TileType::O,
                TileType::T,
                TileType::S,
                TileType::Z,
                TileType::J,
                TileType::L,
            ]);
            len = 7;
        }

        if len == 1 {
            return self.types.pop().unwrap();
        }

        let value = (self.rng.next_u32() as usize) % len;
        self.types.swap_remove(value)
    }
}

struct RandomNES {
    rng: StdRng,
    last: TileType,
}

impl RandomNES {
    fn new(seed: [u8; 32]) -> RandomNES {
        RandomNES {
            rng: StdRng::from_seed(seed),
            last: TileType::Empty,
        }
    }
}

impl RandomGenerator for RandomNES {
    fn next(&mut self) -> TileType {
        let value = (self.rng.next_u32() as usize) % 8;
        let next = TileType::from_usize(value);

        if next == TileType::Empty || next == self.last {
            let value = (self.rng.next_u32() as usize) % 7;
            let next = TileType::from_usize(value);

            self.last = next;
            return next;
        }

        self.last = next;
        next
    }
}

pub fn create(seed: [u8; 32], t: RandomGeneratorType) -> Box<dyn RandomGenerator> {
    match t {
        RandomGeneratorType::RandomBag => Box::new(RandomBag::new(seed)),
        RandomGeneratorType::RandomNES => Box::new(RandomNES::new(seed)),
    }
}
