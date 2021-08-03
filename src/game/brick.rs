use rand::prelude::*;

use super::actions::Rotation;
const BRICK_CONFIGS: [([u8; 16], usize); 7] = [
    ([0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0], 4),
    ([0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3),
    ([1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3),
    ([0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3),
    ([1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3),
    ([1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 2),
    ([0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 3),
];

#[derive(Clone, Copy)]
pub struct Brick
{
    pub len: usize,
    content: [u8; 16],
    pub id: usize,
}

impl Brick
{
    pub fn new(config_index: usize) -> Self
    {
        Self {
            len: BRICK_CONFIGS[config_index].1,
            content: BRICK_CONFIGS[config_index].0,
            id: config_index,
        }
    }

    pub fn new_rand() -> Self
    {
        static mut PERMUTATION: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
        static mut PERMUTE_INDEX: usize = 7;

        let result;

        unsafe {
            if PERMUTE_INDEX >= 7
            {
                PERMUTE_INDEX = 0;
                PERMUTATION.shuffle(&mut rand::thread_rng());
            }

            result = PERMUTATION[PERMUTE_INDEX];

            PERMUTE_INDEX += 1;
        }

        Self::new(result)
    }

    pub fn get(&self, x: usize, y: usize) -> u8
    {
        self.content[x + y * self.len]
    }

    fn transpose(&mut self)
    {
        for x in 0..self.len
        {
            for y in 0..self.len
            {
                if x > y
                {
                    self.content.swap(x + y * self.len, y + x * self.len);
                }
            }
        }
    }

    pub fn rotate(&mut self, rotation: Rotation)
    {
        self.transpose();

        match rotation
        {
            Rotation::Clockwise =>
            {
                // Reverse rows
                for y in 0..self.len
                {
                    for x in 0..(self.len / 2)
                    {
                        self.content
                            .swap(x + y * self.len, (self.len - 1 - x) + y * self.len);
                    }
                }
            }
            Rotation::CounterClockwise =>
            {
                // Reverse columns
                for x in 0..self.len
                {
                    for y in 0..(self.len / 2)
                    {
                        self.content
                            .swap(x + y * self.len, x + (self.len - 1 - y) * self.len);
                    }
                }
            }
        }
    }
}
