use std::cell::RefCell;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::cauldron::Cauldron;
use crate::chip::{Chip, Color};
use crate::player::Player;

pub trait Rule {
    fn chip_drawn(&self, player: &Player, cauldron: &mut Cauldron);
}

pub trait Strategy {
    fn name(&self) -> String;
    fn continue_drawing(&self, player: &Player, bag: &Vec<Chip>, cauldron: &Cauldron) -> bool;
    fn spend_flask(&self, player: &Player, bag: &Vec<Chip>, cauldron: &Cauldron) -> bool;
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

    pub fn round(&self, player: &mut Player) -> Cauldron {
        let strategy = player.strategy();
        let rat = 0;
        let mut bag = player.bag().clone();
        let mut cauldron = Cauldron::new(player.drop_position() + rat);

        loop {
            let chip = self.draw(&mut bag);
            cauldron.add_chip(chip);

            self.apply_chip_drawn_rules(player, &mut bag, &mut cauldron);

            if cauldron.is_exploded() {
                break;
            }

            if player.flask() && cauldron.last_chip().map(|c| c.color()) == Some(Color::White) {
                if strategy.spend_flask(player, &bag, &cauldron) {
                    // let chip = cauldron.remove_last();
                    todo!()
                }
            }

            if !strategy.continue_drawing(player, &bag, &cauldron) {
                break;
            }
        }

        while !cauldron.is_exploded() && !cauldron.is_full() {}

        cauldron
    }

    fn apply_chip_drawn_rules(
        &self,
        player: &Player,
        bag: &mut Vec<Chip>,
        cauldron: &mut Cauldron,
    ) {
        for rule in self.rules.iter() {
            rule.chip_drawn(player, cauldron)
        }
    }
}
