use std::rc::Rc;

use rand::Rng;

mod cauldron;
mod chip;
mod game;
mod player;

use cauldron::Cauldron;
use chip::Chip;
use game::{Game, Strategy};
use player::Player;

struct SimpleStrategy {}

impl Strategy for SimpleStrategy {
    fn name(&self) -> String {
        String::from("SimpleStrategy")
    }

    fn continue_drawing(&self, _: &Game, _: &Player, bag: &Vec<Chip>, cauldron: &Cauldron) -> bool {
        cauldron.chance_to_explode(bag) < 0.5
    }

    fn spend_flask(&self, game: &Game, _: &Player, bag: &Vec<Chip>, cauldron: &Cauldron) -> bool {
        if cauldron.chance_to_explode(bag) > 0.0 {
            // spend falsk sometimes, but only when about to explode
            game.rng().gen_bool(0.5)
        } else {
            false
        }
    }

    fn buy_instead_of_points(&self, game: &Game, _: &Player, _: &Cauldron) -> bool {
        game.rng().gen_bool(0.5)
    }
}

pub fn main() {
    let players = vec![Player::new(Rc::new(SimpleStrategy {}))];
    let mut game = Game::new(players);
    game.round();
}
