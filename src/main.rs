use std::cell::RefCell;
use std::rc::Rc;

use bonus_die::BonusDie;
use rand::{rngs::SmallRng, Rng, SeedableRng};

mod bonus_die;
mod cauldron;
mod chip;
mod game;
mod player;
mod rules;
mod strategy;

use chip::Chip;
use game::{run, Game};
use player::Player;
use rules::RuleSet;
use strategy::Strategy;

struct SimpleStrategy {
    rng: Rc<RefCell<SmallRng>>,
}

impl SimpleStrategy {
    fn new(rng: Rc<RefCell<SmallRng>>) -> Self {
        Self { rng }
    }
}

impl Strategy for SimpleStrategy {
    fn name(&self) -> String {
        String::from("SimpleStrategy")
    }

    fn continue_drawing(&self, _: &Game, player: &Player) -> bool {
        let change = player.cauldron().chance_to_explode(player.bag());
        self.rng.borrow_mut().gen_bool((1.0 - change).into())
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
        game: &Game,
        player: &Player,
        options: &Vec<Vec<Chip>>,
    ) -> Option<usize> {
        Some(0)
    }
}

pub fn main() {
    let rng = Rc::new(RefCell::new(SmallRng::from_entropy()));
    let bonus_die = BonusDie::new(rng.clone());

    let players = vec![Player::new(
        "Player 1",
        rng.clone(),
        Rc::new(SimpleStrategy::new(rng.clone())),
    )];

    let rules = RuleSet::new(vec![
        Rc::new(rules::core::Orange),
        Rc::new(rules::set1::Green),
    ]);

    let mut game = Game::new(players, rules, bonus_die);

    run(&mut game);
}
