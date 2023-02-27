use std::cell::RefCell;
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng};

use super::Strategy;
use crate::chip::{Chip, Color};
use crate::game::Game;
use crate::player::Player;

pub struct PreferColorStrategy {
    preferred_color: Color,
    rng: Rc<RefCell<SmallRng>>,
}

impl PreferColorStrategy {
    pub fn new(preferred_color: Color, rng: Rc<RefCell<SmallRng>>) -> Self {
        Self {
            rng,
            preferred_color,
        }
    }
}

impl Strategy for PreferColorStrategy {
    fn name(&self) -> String {
        format!("PreferColorStrategy({col:?})", col = self.preferred_color)
    }

    fn continue_drawing(&self, _: &Game, player: &Player) -> bool {
        let change = player.cauldron().chance_to_explode(player.bag());
        if change < 1.0 {
            self.rng.borrow_mut().gen_bool((1.0 - change).into())
        } else {
            false
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
        // self.rng.borrow_mut().gen_bool(0.5)
        true
    }

    fn choose_chips_to_add_to_bag(
        &self,
        _: &Game,
        _: &Player,
        choices: &Vec<Vec<Chip>>,
    ) -> Option<usize> {
        let index_of_first_preferred = choices
            .iter()
            .enumerate()
            .filter(|(_, chips)| chips.iter().any(|c| c.color() == self.preferred_color))
            .map(|(idx, _)| idx)
            .next();
        return index_of_first_preferred.or(Some(0));
    }

    fn choose_chips_to_add_to_cauldon(&self, _: &Player, chips: &Vec<Chip>) -> Option<usize> {
        let non_white_choices: Vec<_> = chips
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, c)| c.color() != Color::White)
            .collect();

        let mut preferred_choices: Vec<_> = non_white_choices
            .iter()
            .cloned()
            .filter(|(_, c)| c.color() == self.preferred_color)
            .collect();

        preferred_choices.sort_by_key(|(_, c)| c.value());

        preferred_choices
            .last()
            .or(non_white_choices.first())
            .map(|(idx, _)| *idx)
    }

    fn wants_to_pay_rubies_to_fill_flask(&self, _: &Game, _: &Player) -> bool {
        // fill the flask sometimes, at other times move a drop
        return self.rng.borrow_mut().gen_bool(0.5);
    }

    fn wants_to_pay_rubies_to_move_drop(&self, game: &Game, player: &Player) -> bool {
        // always buy if there is a discount or has plenty
        if game.rubies_to_move_drop < 2 || player.rubies() > 2 {
            return true;
        }
        return self.rng.borrow_mut().gen_bool(0.5);
    }
}