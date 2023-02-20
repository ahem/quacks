use std::cell::RefCell;
use std::rc::Rc;

use rand::{rngs::SmallRng, Rng};

pub struct BonusDie {
    rng: Rc<RefCell<SmallRng>>,
}

#[derive(Debug, Clone, Copy)]
pub enum BonusDieResult {
    OrangeChip,
    Ruby,
    VictoryPoint(u8),
    Drop,
}

impl BonusDie {
    pub fn new(rng: Rc<RefCell<SmallRng>>) -> Self {
        Self { rng }
    }

    pub fn roll(&self) -> BonusDieResult {
        let result = match self.rng.borrow_mut().gen_range(1..=6) {
            1 => BonusDieResult::OrangeChip,
            2 => BonusDieResult::Ruby,
            3 | 4 => BonusDieResult::VictoryPoint(1),
            5 => BonusDieResult::VictoryPoint(2),
            6 => BonusDieResult::Drop,
            _ => unreachable!(),
        };
        println!("bonus die result: {result:?}");
        result
    }
}
