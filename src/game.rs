use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::chip::{Chip, Color};
use crate::player::Player;

pub trait Rule {
    fn chip_drawn(&self, player: &mut Player);
    fn cauldron_finished(&self, player: &mut Player);
    fn black_chip(&self, player: &mut Player);
    fn green_chip(&self, player: &mut Player);
    fn purple_chip(&self, player: &mut Player);
}

pub trait Strategy {
    fn name(&self) -> String;
    fn continue_drawing(&self, game: &Game, player: &Player) -> bool;

    fn spend_flask(&self, game: &Game, player: &Player) -> bool;

    fn buy_instead_of_points(&self, game: &Game, player: &Player) -> bool;
}

pub struct Game {
    rng: RefCell<SmallRng>,
    players: Vec<Rc<RefCell<Player>>>,
    rules: Vec<Box<dyn Rule>>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let players = players
            .into_iter()
            .map(|p| Rc::new(RefCell::new(p)))
            .collect();
        let rng = std::cell::RefCell::new(SmallRng::from_entropy());
        Self {
            rng,
            players,
            rules: vec![],
        }
    }

    pub fn draw(&self, bag: &mut Vec<Chip>) -> Chip {
        let mut rng = self.rng.borrow_mut();
        bag.remove(rng.gen_range(0..bag.len()))
    }

    fn calculate_rat(&self, _player_idx: &Player) -> u8 {
        0 // TODO
    }

    fn fill_cauldron_phase(&mut self, player: &mut Player) {
        let strategy = player.strategy();

        let drop_position = player.drop_position();
        let rat_position = self.calculate_rat(&player);

        loop {
            let chip = self.draw(player.bag_mut());
            player.cauldron_mut().add_chip(chip);

            self.apply_chip_drawn_rules(player);

            if player.cauldron().is_exploded() {
                println!("  exploded");
                break;
            }

            if player.flask()
                && player.cauldron().last_chip().map(|c| c.color()) == Some(Color::White)
            {
                if strategy.spend_flask(self, player) {
                    let chip = player.cauldron_mut().remove_last().unwrap();
                    player.bag_mut().push(chip);
                    player.spend_flask();
                    println!("  spend flask!");
                }
            }

            if !strategy.continue_drawing(self, player) {
                println!("  stopped drawing");
                break;
            }
        }
    }

    pub fn buy_chips_phase(&self, player: &mut Player, coins: u8) {
        // TODO!
    }

    pub fn spend_rubies_phase(&self, player: &mut Player) {
        // TODO!
    }

    pub fn round(&mut self) {
        let mut cauldrons = vec![];
        for i in 0..self.players.len() {
            let player = self.players[i].clone();
            let cauldron = self.fill_cauldron_phase(&mut player.borrow_mut());
            cauldrons.push((player, cauldron));
        }

        for (player, cauldron) in &mut cauldrons {
            self.apply_cauldron_finished_rules(&mut player.borrow_mut());
        }

        println!("{cauldrons:?}");

        // TODO: bonus die

        for (player, cauldron) in &mut cauldrons {
            self.apply_black_chip_rule(&mut player.borrow_mut());
            self.apply_green_chip_rule(&mut player.borrow_mut());
            self.apply_purple_chip_rule(&mut player.borrow_mut());
        }

        for (player, cauldron) in &cauldrons {
            let mut player = player.borrow_mut();
            let score = player.cauldron().score();
            if score.ruby {
                player.add_rubies(1);
            }
            if !player.cauldron().is_exploded() {
                self.buy_chips_phase(&mut player, score.coins);
                player.add_victory_points(score.points);
            } else {
                let strategy = player.strategy().clone();
                if strategy.buy_instead_of_points(&self, &player) {
                    self.buy_chips_phase(&mut player, score.coins);
                } else {
                    player.add_victory_points(score.points);
                }
            }
        }

        for player in self.players.iter().cloned() {
            self.spend_rubies_phase(&mut player.borrow_mut());
        }
    }

    pub fn rng(&self) -> RefMut<SmallRng> {
        self.rng.borrow_mut()
    }

    fn apply_chip_drawn_rules(&self, player: &mut Player) {
        for rule in self.rules.iter() {
            rule.chip_drawn(player)
        }
    }

    fn apply_cauldron_finished_rules(&self, player: &mut Player) {
        for rule in self.rules.iter() {
            rule.cauldron_finished(player)
        }
    }

    fn apply_black_chip_rule(&self, player: &mut Player) {
        for rule in self.rules.iter() {
            rule.black_chip(player)
        }
    }

    fn apply_green_chip_rule(&self, player: &mut Player) {
        for rule in self.rules.iter() {
            rule.green_chip(player)
        }
    }

    fn apply_purple_chip_rule(&self, player: &mut Player) {
        for rule in self.rules.iter() {
            rule.purple_chip(player)
        }
    }
}
