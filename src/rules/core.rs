use std::{cell::RefCell, rc::Rc};

use crate::{
    chip::{Chip, Color},
    game::Game,
    player::Player,
    rules::Rule,
};

pub struct Orange;

impl Rule for Orange {
    fn purchase_options(&self, _: &Game) -> Vec<(Chip, u8)> {
        vec![(Chip::Orange1, 3)]
    }
}

pub struct Black;

impl Rule for Black {
    fn purchase_options(&self, _: &Game) -> Vec<(Chip, u8)> {
        vec![(Chip::Black1, 10)]
    }

    fn black_chip(&self, player: Rc<RefCell<Player>>, game: &Game) {
        let own_count = player.borrow().cauldron().number_of(Color::Black);
        let idx = game.player_position(&player);
        let number_of_players = game.players().len();

        if number_of_players == 2 {
            let other_count = game.players()[(idx + 1) % 2]
                .cauldron()
                .number_of(Color::Black)
                .to_owned();
            if own_count >= other_count {
                player.borrow_mut().move_drop();
            }
            if own_count > other_count {
                player.borrow_mut().add_rubies(1);
            }
        } else if number_of_players > 2 {
            let left_count = game.players()[(idx + number_of_players - 1) % number_of_players]
                .cauldron()
                .number_of(Color::Black);
            let right_count = game.players()[(idx + 1) % number_of_players]
                .cauldron()
                .number_of(Color::Black);
            if own_count > left_count || own_count > right_count {
                player.borrow_mut().move_drop();
            }
            if own_count > left_count && own_count > right_count {
                player.borrow_mut().add_rubies(1);
            }
        }
    }
}
