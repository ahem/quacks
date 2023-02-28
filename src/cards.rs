// done: Roll the die: "Everyone rolls the bonus die and gets the bonus shown" (purple)
// TODO: A Second Change: "After you have placed the first 5 chips in your pot, choose to continue OR begin the round all over again - but you get this choice only once" (blue)
// done: Pumpkin Patch Party: "In this round, every orange chip is moved 1 extra space forward" (blue)
// TODO: You Only Get to Choose One: "Choose: Take 1 black chip OR take any one 2-chip OR 3 rubies" (purple)
// TODO: Choose wisely: "Choose: move droplet 2 spaces forward, OR take 1 purple chip" (purple)
// TODO: An Opportunistic Moment: "Draw 4 chips from your bag. You may trade 1 of them for a chip of the same color with the next higher value. If you can't make a trade, take 1 green chip. Put all the chips back in the bag" (purple)
// TODO: Less is More: "All players draw 5 chips. The player(s) with the lowest sum take(s) 1 blue 2-chip. All other players receive 1 ruby. Put all the chips back in the bag." (purple)
// TODO: It's Shining Extra Bright: "If you reach a scoring space with a ruby, you get an extra ruby." (blue)
// TODO: Strong Ingredient: "Beginning with the start player, if you stopped without an explosion, draw up to 5 chips from your bag. You may place 1 of them in your pot." (blue)
// TODO: Lucky Devil: "If you reach a scoring space with a ruby this round, you get an extra 2 victory points, even if your pot has exploded." (blue)
// TODO: The Pot is Full: "Any player who gets to roll the die this turn round rolls twice" (blue)
// TODO: Living in Luxury: "The threshold for white chips is raised in this round from 7 to 9" (blue)
// TODO: The Pot is Filling Up: "Move your droplet 1 space forward" (purple)
// TODO: Beginner's Bonus: "The player(s) with the fewest victory points receive(s) 1 green 1-chip" (purple)
// TODO: Rats are your Friends: "Choose: Take any one 4-chip OR 1 victory point for each rat tail you get this turn." (purple)"
// TODO: Just in Time: "Choose: Take 4 vicotry points OR remove 1 white 1-chip from your bag" (purple)
// TODO: Charity: "The player(s) with the fewest rubies receive(s) 1 ruby." (purple)
// TODO: Well Strirred: "In this round, you may put the first white chip you draw back into the bag." (blue)

use std::cell::RefCell;
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng};

use crate::bonus_die::{BonusDie, BonusDieResult};
use crate::chip::Chip;
use crate::game::Game;
use crate::player::Player;
use crate::rules::Rule;

struct RollTheDie {
    bonus_die: BonusDie,
}
impl Rule for RollTheDie {
    fn bonus_die_rolled(&self, player: Rc<RefCell<Player>>, _: BonusDieResult) {
        let roll = self.bonus_die.roll();
        match roll {
            BonusDieResult::OrangeChip => {
                player.borrow_mut().bag_mut().push(Chip::Orange1);
            }
            BonusDieResult::VictoryPoint(points) => {
                player.borrow_mut().add_victory_points(points);
            }
            BonusDieResult::Ruby => {
                player.borrow_mut().add_rubies(1);
            }
            BonusDieResult::Drop => {
                player.borrow_mut().move_drop();
            }
        }
    }
}

struct PumpkinPatchParty;
impl Rule for PumpkinPatchParty {
    fn orange_chip_drawn(&self, player: Rc<RefCell<Player>>, _: &Game, _: u8) {
        player.borrow_mut().cauldron_mut().increase_position(1)
    }
}

struct LivingInLuxury;
impl Rule for LivingInLuxury {}

pub struct FortuneTellerCards {
    cards: Vec<Box<dyn Rule>>,
    rng: Rc<RefCell<SmallRng>>,
}

impl FortuneTellerCards {
    pub fn new(rng: Rc<RefCell<SmallRng>>) -> Self {
        Self {
            #[rustfmt::skip]
            cards: vec![
                Box::new(RollTheDie { bonus_die: BonusDie::new(rng.clone()) }),
                Box::new(PumpkinPatchParty),
            ],
            rng,
        }
    }

    pub fn pick(&mut self) -> Box<dyn Rule> {
        let idx = self.rng.borrow_mut().gen_range(0..self.cards.len());
        self.cards.remove(idx)
    }
}
