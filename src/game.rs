use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::cauldron::Cauldron;
use crate::chip::{Chip, Color};
use crate::player::Player;

pub trait Rule {
    fn chip_drawn(&self, player: &Player, cauldron: &mut Cauldron, bag: &mut Vec<Chip>);
    fn cauldron_finished(&self, player: &Player, cauldron: &mut Cauldron);
    fn black_chip(&self, player: &mut Player, cauldron: &mut Cauldron);
    fn green_chip(&self, player: &mut Player, cauldron: &mut Cauldron);
    fn purple_chip(&self, player: &mut Player, cauldron: &mut Cauldron);
}

pub trait Strategy {
    fn name(&self) -> String;
    fn continue_drawing(
        &self,
        game: &Game,
        player: &Player,
        bag: &Vec<Chip>,
        cauldron: &Cauldron,
    ) -> bool;

    fn spend_flask(
        &self,
        game: &Game,
        player: &Player,
        bag: &Vec<Chip>,
        cauldron: &Cauldron,
    ) -> bool;

    fn buy_instead_of_points(&self, game: &Game, player: &Player, cauldron: &Cauldron) -> bool;
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

    fn fill_cauldron_phase(&mut self, player: &mut Player) -> Cauldron {
        let strategy = player.strategy();
        let mut bag = player.bag().clone();

        let drop_position = player.drop_position();
        let rat_position = self.calculate_rat(&player);
        let mut cauldron = Cauldron::new(drop_position, rat_position);

        loop {
            let chip = self.draw(&mut bag);
            cauldron.add_chip(chip);

            self.apply_chip_drawn_rules(&player, &mut bag, &mut cauldron);

            if cauldron.is_exploded() {
                println!("  exploded");
                break;
            }

            if player.flask() && cauldron.last_chip().map(|c| c.color()) == Some(Color::White) {
                if strategy.spend_flask(self, player, &bag, &cauldron) {
                    let chip = cauldron.remove_last().unwrap();
                    bag.push(chip);
                    player.spend_flask();
                    println!("  spend flask!");
                }
            }

            if !strategy.continue_drawing(self, player, &bag, &cauldron) {
                println!("  stopped drawing");
                break;
            }
        }

        cauldron
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
            self.apply_cauldron_finished_rules(&player.borrow(), cauldron);
        }

        println!("{cauldrons:?}");

        // TODO: bonus die

        for (player, cauldron) in &mut cauldrons {
            self.apply_black_chip_rule(&mut player.borrow_mut(), cauldron);
            self.apply_green_chip_rule(&mut player.borrow_mut(), cauldron);
            self.apply_purple_chip_rule(&mut player.borrow_mut(), cauldron);
        }

        for (player, cauldron) in &cauldrons {
            let mut player = player.borrow_mut();
            let score = cauldron.score();
            if score.ruby {
                player.add_rubies(1);
            }
            if !cauldron.is_exploded() {
                self.buy_chips_phase(&mut player, score.coins);
                player.add_victory_points(score.points);
            } else {
                let strategy = player.strategy().clone();
                if strategy.buy_instead_of_points(&self, &player, &cauldron) {
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

    fn apply_chip_drawn_rules(
        &self,
        player: &Player,
        bag: &mut Vec<Chip>,
        cauldron: &mut Cauldron,
    ) {
        for rule in self.rules.iter() {
            rule.chip_drawn(player, cauldron, bag)
        }
    }

    fn apply_cauldron_finished_rules(&self, player: &Player, cauldron: &mut Cauldron) {
        for rule in self.rules.iter() {
            rule.cauldron_finished(player, cauldron)
        }
    }

    fn apply_black_chip_rule(&self, player: &mut Player, cauldron: &mut Cauldron) {
        for rule in self.rules.iter() {
            rule.black_chip(player, cauldron)
        }
    }

    fn apply_green_chip_rule(&self, player: &mut Player, cauldron: &mut Cauldron) {
        for rule in self.rules.iter() {
            rule.green_chip(player, cauldron)
        }
    }

    fn apply_purple_chip_rule(&self, player: &mut Player, cauldron: &mut Cauldron) {
        for rule in self.rules.iter() {
            rule.purple_chip(player, cauldron)
        }
    }
}
