use std::rc::Rc;

mod bonus_die;
mod cauldron;
mod chip;
mod game;
mod player;
mod rules;
mod strategy;

use bonus_die::BonusDie;
use chip::Color;
use game::{run, Game};
use player::Player;
use rules::RuleSet;
use strategy::{prefer_color::PreferColorStrategy, simple::SimpleStrategy};

mod rng {
    use rand::{rngs::SmallRng, Rng, SeedableRng};
    use std::{cell::RefCell, rc::Rc};
    pub fn create_rng(seed: Option<u64>) -> Rc<RefCell<SmallRng>> {
        let seed = seed.unwrap_or_else(|| SmallRng::from_entropy().gen());
        log::info!("Seed: {seed}");
        Rc::new(RefCell::new(SmallRng::seed_from_u64(seed)))
    }
}

pub fn main() {
    env_logger::init();
    let mut wins = vec![0, 0];
    for _ in 0..10000 {
        let rng = rng::create_rng(None);
        let rules = RuleSet::new(vec![
            Rc::new(rules::core::Black),
            Rc::new(rules::core::Orange),
            Rc::new(rules::set1::Blue),
            Rc::new(rules::set1::Green),
            Rc::new(rules::set1::Red),
            Rc::new(rules::set1::Yellow),
            Rc::new(rules::set1::Purple),
        ]);
        let players = vec![
            Player::new(
                "Player 1",
                rng.clone(),
                Rc::new(SimpleStrategy::new(rng.clone())),
            ),
            Player::new(
                "Player 2",
                rng.clone(),
                Rc::new(PreferColorStrategy::new(Color::Blue, rng.clone())),
            ),
        ];

        let mut game = Game::new(players.clone(), rules.clone(), BonusDie::new(rng.clone()));
        run(&mut game);

        let mut results: Vec<_> = game
            .players()
            .iter()
            .map(|p| p.victory_points())
            .enumerate()
            .collect();
        results.sort_by_key(|(_, score)| *score);
        wins[results[0].0] += 1;
    }
    for (idx, wins) in wins.iter().enumerate() {
        println!("player {n} won {wins} times", n = idx + 1);
    }
}
