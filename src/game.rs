use std::cell::RefCell;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::cauldron::Cauldron;
use crate::chip::Chip;
use crate::player::Player;

pub trait Rule {
    fn chip_drawn(&self, player: &Player, chip: Chip, cauldron: &mut Cauldron);
}

pub trait Strategy {
    fn continue_drawing(&self, player: &Player, bag: &Vec<Chip>, cauldron: &Cauldron) -> bool;
}

pub struct Game {
    rng: RefCell<SmallRng>,
    rules: Vec<Box<dyn Rule>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            rng: std::cell::RefCell::new(SmallRng::from_entropy()),
            rules: vec![],
        }
    }

    pub fn draw(&self, bag: &mut Vec<Chip>) -> Chip {
        let mut rng = self.rng.borrow_mut();
        bag.remove(rng.gen_range(0..bag.len()))
    }

    pub fn round(&self, player: &mut Player, strategy: &impl Strategy) -> Cauldron {
        let mut bag = player.bag().clone();
        let rat = 0;
        let mut cauldron = Cauldron::new(player.drop_position() + rat);

        while !cauldron.is_exploded() && !cauldron.is_full() {
            let chip = self.draw(&mut bag);
            cauldron.add_chip(chip);

            if !strategy.continue_drawing(player, &bag, &cauldron) {
                break;
            }
        }

        cauldron
    }

    fn apply_chip_drawn_rules(&self, player: &Player, chip: Chip, cauldron: &mut Cauldron) {
        for rule in self.rules.iter() {
            rule.chip_drawn(player, chip, cauldron)
        }
    }
}
