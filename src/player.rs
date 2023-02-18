use std::fmt::Debug;
use std::rc::Rc;

use crate::{cauldron::Cauldron, chip::Chip, game::Strategy};

pub struct Player {
    score: u16,
    rubies: u8,
    flask: bool,
    drop: u8,
    bag: Vec<Chip>,
    cauldron: Cauldron,
    strategy: Rc<dyn Strategy>,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("bag", &self.bag)
            .field("flash", &self.flask)
            .field("drop", &self.drop)
            .field("strategy", &self.strategy.name())
            .finish()
    }
}

impl Player {
    pub fn new(strategy: Rc<dyn Strategy>) -> Self {
        Self {
            strategy,
            score: 0,
            rubies: 0,
            flask: true,
            drop: 0,
            cauldron: Cauldron::new(),
            bag: vec![
                Chip::White1,
                Chip::White1,
                Chip::White1,
                Chip::White1,
                Chip::White2,
                Chip::White2,
                Chip::White3,
                Chip::Orange1,
                Chip::Green1,
            ],
        }
    }

    pub fn cauldron(&self) -> &Cauldron {
        return &self.cauldron;
    }

    pub fn cauldron_mut(&mut self) -> &mut Cauldron {
        return &mut self.cauldron;
    }

    pub fn bag(&self) -> &Vec<Chip> {
        &self.bag
    }

    pub fn bag_mut(&mut self) -> &mut Vec<Chip> {
        &mut self.bag
    }

    pub fn flask(&self) -> bool {
        self.flask
    }

    pub fn spend_flask(&mut self) {
        self.flask = false;
    }

    pub fn drop_position(&self) -> u8 {
        self.drop
    }

    pub fn strategy(&self) -> Rc<dyn Strategy> {
        self.strategy.clone()
    }

    pub fn add_rubies(&mut self, cnt: u8) {
        self.rubies += cnt;
    }

    pub fn add_victory_points(&mut self, points: u8) {
        self.score = self.score + u16::from(points);
    }
}
