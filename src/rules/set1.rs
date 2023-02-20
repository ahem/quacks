use std::{cell::RefCell, rc::Rc};

use crate::{
    chip::{Chip, Color},
    game::Game,
    player::Player,
    rules::Rule,
};

pub struct Green;

impl Rule for Green {
    fn green_chip(&self, player: Rc<RefCell<Player>>) {
        let cnt = player
            .borrow()
            .cauldron()
            .chips()
            .iter()
            .rev()
            .take(2)
            .filter(|c| c.color() == Color::Green)
            .count();
        if cnt > 0 {
            println!(
                "{player} recieves {cnt} rubies from green chips",
                player = player.borrow()
            );
        }
        player.borrow_mut().add_rubies(cnt as u8)
    }

    fn purchase_options(&self) -> Vec<(Chip, u8)> {
        vec![(Chip::Green1, 4), (Chip::Green2, 8), (Chip::Green4, 14)]
    }
}

pub struct Red;

impl Rule for Red {
    fn purchase_options(&self) -> Vec<(Chip, u8)> {
        vec![(Chip::Red1, 6), (Chip::Red2, 10), (Chip::Red4, 16)]
    }

    fn red_chip_drawn(&self, player: Rc<RefCell<Player>>, _: &Game, _value: u8) {
        let cnt = player.borrow().cauldron().number_of(Color::Orange);
        if cnt == 1 || cnt == 2 {
            player.borrow_mut().cauldron_mut().increase_position(1);
        }
        if cnt > 2 {
            player.borrow_mut().cauldron_mut().increase_position(2);
        }
    }
}

pub struct Blue;

impl Rule for Blue {
    fn purchase_options(&self) -> Vec<(Chip, u8)> {
        vec![(Chip::Blue1, 5), (Chip::Blue2, 10), (Chip::Blue4, 19)]
    }

    fn blue_chip_drawn(&self, player: Rc<RefCell<Player>>, game: &Game, value: u8) {
        let mut chips = vec![];
        for _ in 0..value {
            chips.push(player.borrow_mut().draw());
        }

        let chip = player
            .borrow()
            .choose_chips_to_add_to_cauldon(&chips)
            .clone()
            .map(|idx| chips.remove(idx));

        // return not-chosen chips
        player.borrow_mut().bag_mut().append(&mut chips);

        // add the chosen one
        if let Some(chip) = chip {
            player.borrow_mut().add_chip_to_cauldron(chip);
            game.rules().apply_chip_drawn_rules(player, game);
        }
    }
}

struct Yellow;

impl Rule for Yellow {}
