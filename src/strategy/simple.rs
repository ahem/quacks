use std::cell::RefCell;
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng};

use super::Strategy;
use crate::chip::{Chip, Color};
use crate::game::Game;
use crate::player::Player;

pub struct SimpleStrategy {
    rng: Rc<RefCell<SmallRng>>,
}

impl SimpleStrategy {
    pub fn new(rng: Rc<RefCell<SmallRng>>) -> Self {
        Self { rng }
    }
}

impl Strategy for SimpleStrategy {
    fn name(&self) -> String {
        String::from("SimpleStrategy")
    }

    fn continue_drawing(&self, game: &Game, player: &Player) -> bool {
        let change = player.cauldron().chance_to_explode(player.bag());

        if game.turn < 4 {
            change < 1.0
        } else if game.turn < 6 {
            self.rng.borrow_mut().gen_bool((1.0 - change).into())
        } else {
            change == 0.0
        }
    }

    fn spend_flask(&self, _: &Game, player: &Player) -> bool {
        if player.cauldron().chance_to_explode(player.bag()) > 0.0 {
            // spend falsk sometimes, but only when about to explode
            self.rng.borrow_mut().gen_bool(0.5)
        } else {
            false
        }
    }

    fn buy_instead_of_points(&self, _: &Game, _player: &Player) -> bool {
        true
    }

    fn choose_chips_to_add_to_bag(
        &self,
        _: &Game,
        _: &Player,
        _: &Vec<Vec<Chip>>,
    ) -> Option<usize> {
        // always just buy the first (most expensive) option
        Some(0)
    }

    fn choose_chips_to_add_to_cauldon(&self, _: &Player, chips: &Vec<Chip>) -> Option<usize> {
        let non_white_choices: Vec<usize> = chips
            .iter()
            .enumerate()
            .filter_map(|(n, c)| (c.color() != Color::White).then_some(n))
            .collect();

        if !non_white_choices.is_empty() {
            let idx = self.rng.borrow_mut().gen_range(0..non_white_choices.len());
            Some(non_white_choices[idx])
        } else {
            None
        }
    }

    fn wants_to_pay_rubies_to_fill_flask(&self, _: &Game, _: &Player) -> bool {
        // fill the flask sometimes, at other times move a drop
        return self.rng.borrow_mut().gen_bool(0.5);
    }

    fn wants_to_pay_rubies_to_move_drop(&self, _: &Game, _: &Player) -> bool {
        return self.rng.borrow_mut().gen_bool(0.5);
    }
}
