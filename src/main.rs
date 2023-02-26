use std::cell::RefCell;
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng, SeedableRng};

mod bonus_die;
mod cauldron;
mod chip;
mod game;
mod player;
mod rules;
mod strategy;

use bonus_die::BonusDie;
use game::{run, Game};
use player::Player;
use rules::RuleSet;
use strategy::{prefer_blue::PreferBlueStrategy, simple::SimpleStrategy};

pub fn create_rng(seed: Option<u64>) -> Rc<RefCell<SmallRng>> {
    let seed = seed.unwrap_or_else(|| SmallRng::from_entropy().gen());
    let seed = 17451425246857276322;
    log::info!("Seed: {seed}");
    Rc::new(RefCell::new(SmallRng::seed_from_u64(seed)))
}

pub fn main() {
    env_logger::init();
    let rng = create_rng(None);
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
