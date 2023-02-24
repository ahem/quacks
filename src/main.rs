use std::cell::RefCell;
use std::rc::Rc;

use bonus_die::BonusDie;
use rand::{rngs::SmallRng, SeedableRng};

mod bonus_die;
mod cauldron;
mod chip;
mod game;
mod player;
mod rules;
mod strategy;

use game::{run, Game};
use player::Player;
use rules::RuleSet;
use strategy::{prefer_blue::PreferBlueStrategy, simple::SimpleStrategy};

pub fn main() {
    let rng = Rc::new(RefCell::new(SmallRng::from_entropy()));
    let bonus_die = BonusDie::new(rng.clone());

    let players = vec![
        Player::new(
            "Player 1",
            rng.clone(),
            Rc::new(SimpleStrategy::new(rng.clone())),
        ),
        Player::new(
            "Player 2",
            rng.clone(),
            Rc::new(PreferBlueStrategy::new(rng.clone())),
        ),
    ];

    let rules = RuleSet::new(vec![
        Rc::new(rules::core::Black),
        Rc::new(rules::core::Orange),
        Rc::new(rules::set1::Blue),
        Rc::new(rules::set1::Green),
        Rc::new(rules::set1::Red),
        Rc::new(rules::set1::Yellow),
        Rc::new(rules::set1::Purple),
    ]);

    let mut game = Game::new(players, rules, bonus_die);

    run(&mut game);
}
