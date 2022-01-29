use crate::bag::Bag;
use crate::chip::{Chip, Color};
use rand::prelude::*;
use std::collections::HashMap;

pub trait Player {
    fn end_turn(&self, bag: &Bag, potion: &Potion) -> bool;
    fn spend_flask(&self, bag: &Bag, potion: &Potion) -> bool;
}

pub struct Flask {
    full: bool,
}

impl Flask {
    pub fn fill(&mut self) {
        self.full = true;
    }

    pub fn drain(&mut self) {
        self.full = false;
    }

    pub fn is_full(&self) -> bool {
        return self.full;
    }
}

pub struct Potion {
    initial: u8,
    chips: HashMap<u8, Chip>,
}

impl Potion {
    pub fn new(initial: u8) -> Potion {
        return Potion {
            initial,
            chips: HashMap::new(),
        };
    }
    pub fn last_idx(&self) -> &u8 {
        return self.chips.keys().max().unwrap_or(&0);
    }

    pub fn last(&self) -> Option<&Chip> {
        return self.chips.get(self.last_idx());
    }

    pub fn remove_last(&mut self) -> Option<Chip> {
        let idx = self.last_idx().to_owned();
        return self.chips.remove(&idx);
    }

    pub fn add_chip(&mut self, chip: Chip) {
        self.chips.insert(self.last_idx() + chip.value(), chip);
    }
    pub fn add_chip_with_bonus(&mut self, chip: Chip, bonus: u8) {
        self.chips
            .insert(self.last_idx() + chip.value() + bonus, chip);
    }

    pub fn count_chips(&self, color: Color) -> usize {
        return self.chips.values().filter(|x| x.color() == color).count();
    }

    pub fn sum_chips(&self, color: Color) -> u8 {
        return self
            .chips
            .values()
            .filter(|x| x.color() == color)
            .map(|x| x.value())
            .sum();
    }
}

pub fn fill_potion<T>(
    potion: &mut Potion,
    bag: &mut Bag,
    flask: &mut Flask,
    rng: &mut rand::rngs::ThreadRng,
    player: &impl Player,
    initial_pos: u8,
) -> Potion {
    loop {
        let chip = bag.draw(rng);

        match chip.color() {
            Color::White => {
                potion.add_chip(chip);

                if flask.is_full() {
                    if player.spend_flask(&bag, &potion) {
                        flask.drain();
                        bag.add_chip(potion.remove_last().unwrap());
                    }
                }
            }
            Color::Orange | Color::Green | Color::Black | Color::Yellow | Color::Purple => {
                potion.add_chip(chip);
            }
            Color::Red => {
                // TODO: search for orange and use add_chip_with_bonus if there is a bonus
                potion.add_chip(chip);
            }
            Color::Blue => {
                potion.add_chip(chip);
                // TODO: draw and choose
            }
        };

        if player.end_turn(&bag, &potion) {
            return potion;
        }
    }
}
