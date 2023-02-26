use crate::chip::Chip;
use crate::game::Game;
use crate::player::Player;

pub mod prefer_blue;
pub mod simple;

pub trait Strategy {
    fn name(&self) -> String;
    fn continue_drawing(&self, game: &Game, player: &Player) -> bool;
    fn spend_flask(&self, game: &Game, player: &Player) -> bool;
    fn buy_instead_of_points(&self, game: &Game, player: &Player) -> bool;
    fn choose_chips_to_add_to_bag(
        &self,
        game: &Game,
        player: &Player,
        options: &Vec<Vec<Chip>>,
    ) -> Option<usize>;
    fn choose_chips_to_add_to_cauldon(&self, player: &Player, chips: &Vec<Chip>) -> Option<usize>;
    fn wants_to_pay_rubies_to_fill_flask(&self, game: &Game, player: &Player) -> bool;
    fn wants_to_pay_rubies_to_move_drop(&self, game: &Game, player: &Player) -> bool;
}
