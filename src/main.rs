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

    fn continue_drawing(&self, _player: &Player, bag: &Vec<Chip>, cauldron: &Cauldron) -> bool {
        cauldron.chance_to_explode(bag) < 0.5
    }
}

pub fn main() {
    let mut player = Player::new(Box::new(SimpleStrategy {}));

    println!("{player:?}");

    let game = Game::new();
    let cauldron = game.round(&mut player);
    let score = cauldron.score();

    println!("{:?} is_exploded: {:?}", score, cauldron.is_exploded());
}
