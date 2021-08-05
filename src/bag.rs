use crate::chip::Chip;
use rand::prelude::*;

pub struct Bag {
    chips: Vec<Chip>,
}

impl Bag {
    pub fn draw(&mut self, rng: &mut rand::rngs::ThreadRng) -> Chip {
        let idx = rng.gen_range(0..self.chips.len());
        return self.chips.remove(idx);
    }

    pub fn add_chip(&mut self, chip: Chip) {
        self.chips.push(chip);
    }
}
