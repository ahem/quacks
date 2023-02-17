use std::fmt::Debug;
use std::rc::Rc;

use crate::{chip::Chip, game::Strategy};

pub struct Player {
    score: u16,
    rubies: u8,
    chips: Vec<Chip>,
    flask: bool,
    drop: u8,
    strategy: Rc<dyn Strategy>,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("chips", &self.chips)
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
            chips: vec![
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

    pub fn bag(&self) -> &Vec<Chip> {
        &self.chips
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
