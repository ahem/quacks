use std::{cell::RefCell, rc::Rc};

use crate::bonus_die::BonusDieResult;
use crate::chip::{Chip, Color};
use crate::game::Game;
use crate::player::Player;

pub mod core;
pub mod set1;

pub trait Rule {
    #![allow(unused)]
    fn cauldron_finished(&self, player: Rc<RefCell<Player>>) {}
    fn bonus_die_rolled(&self, player: Rc<RefCell<Player>>, result: BonusDieResult) {}
    fn red_chip_drawn(&self, player: Rc<RefCell<Player>>, game: &Game, value: u8) {}
    fn blue_chip_drawn(&self, player: Rc<RefCell<Player>>, game: &Game, value: u8) {}
    fn yellow_chip_drawn(&self, player: Rc<RefCell<Player>>, game: &Game, value: u8) {}
    fn black_chip(&self, player: Rc<RefCell<Player>>, game: &Game) {}
    fn green_chip(&self, player: Rc<RefCell<Player>>) {}
    fn purple_chip(&self, player: Rc<RefCell<Player>>) {}
    fn purchase_options(&self, game: &Game) -> Vec<(Chip, u8)> {
        vec![]
    }
}

#[derive(Clone)]
pub struct RuleSet {
    rules: Vec<Rc<dyn Rule>>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rc<dyn Rule>>) -> Self {
        Self { rules }
    }

    /*
    pub fn add_rule(&mut self, rule: Rc<dyn Rule>) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, rule: &Rc<dyn Rule>) {
        if let Some(idx) = self.rules.iter().position(|x| Rc::ptr_eq(x, rule)) {
            self.rules.remove(idx);
        }
    }
    */

    pub fn apply_chip_drawn_rules(&self, player: Rc<RefCell<Player>>, game: &Game) {
        for rule in self.rules.iter() {
            let chip = player.borrow().cauldron().last_chip().unwrap();
            match chip.color() {
                Color::Red => rule.red_chip_drawn(player.clone(), game, chip.value()),
                Color::Blue => rule.blue_chip_drawn(player.clone(), game, chip.value()),
                Color::Yellow => rule.yellow_chip_drawn(player.clone(), game, chip.value()),
                _ => (),
            }
        }
    }

    pub fn apply_cauldron_finished_rules(&self, player: &Rc<RefCell<Player>>) {
        for rule in self.rules.iter() {
            rule.cauldron_finished(player.clone())
        }
    }

    pub fn apply_black_chip_rules(&self, player: &Rc<RefCell<Player>>, game: &Game) {
        for rule in self.rules.iter() {
            rule.black_chip(player.clone(), game)
        }
    }

    pub fn apply_green_chip_rules(&self, player: &Rc<RefCell<Player>>) {
        for rule in self.rules.iter() {
            rule.green_chip(player.clone())
        }
    }

    pub fn apply_purple_chip_rules(&self, player: &Rc<RefCell<Player>>) {
        for rule in self.rules.iter() {
            rule.purple_chip(player.clone())
        }
    }

    pub fn apply_bonus_die_rolled_rules(
        &self,
        player: &Rc<RefCell<Player>>,
        result: BonusDieResult,
    ) {
        for rule in self.rules.iter() {
            rule.bonus_die_rolled(player.clone(), result);
        }
    }

    pub fn purchase_options(&self, game: &Game, coins: u8) -> Vec<Vec<Chip>> {
        let all_options: Vec<_> = self
            .rules
            .iter()
            .flat_map(|x| x.purchase_options(game))
            .collect();

        let mut all_choices = vec![];

        for (idx, (chip, price)) in all_options.iter().enumerate() {
            for (other_chip, other_price) in all_options[idx + 1..].iter() {
                if other_chip.color() != chip.color() && price + other_price < coins {
                    let mut choice = vec![*chip, *other_chip];
                    choice.sort();
                    all_choices.push((choice, price + other_price));
                }
            }

            let exists = all_choices
                .iter()
                .find(|(choice, _)| choice.iter().find(|c| *c == chip).is_some())
                .is_some();

            if !exists && *price < coins {
                all_choices.push((vec![*chip], *price));
            }
        }

        all_choices.sort();
        all_choices.dedup();
        all_choices.sort_by_key(|(_choice, price)| *price);

        all_choices
            .iter()
            .rev()
            .map(|(choice, _)| choice.clone())
            .collect()
    }
}
