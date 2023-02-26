use std::fmt::Debug;
use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

use rand::{rngs::SmallRng, Rng};

use crate::{cauldron::Cauldron, chip::Chip, chip::Color, game::Game, strategy::Strategy};

pub struct Player {
    name: String,
    score: u16,
    rubies: u8,
    flask: bool,
    drop: u8,
    bag: Vec<Chip>,
    cauldron: Cauldron,
    strategy: Rc<dyn Strategy>,
    rng: Rc<RefCell<SmallRng>>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{name} ({strategy})",
            name = self.name,
            strategy = self.strategy.name()
        ))
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("name", &self.name)
            .field("strategy", &self.strategy.name())
            .field("score", &self.score)
            .field("rubies", &self.rubies)
            .field("flask", &self.flask)
            .field("drop", &self.drop)
            .field("bag", &self.bag)
            .finish()
    }
}

impl Player {
    pub fn new(name: &str, rng: Rc<RefCell<SmallRng>>, strategy: Rc<dyn Strategy>) -> Self {
        Self {
            name: name.to_string(),
            rng,
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

    pub fn draw(&mut self) -> Chip {
        let mut rng = self.rng.borrow_mut();
        self.bag.remove(rng.gen_range(0..self.bag.len()))
    }

    pub fn cauldron(&self) -> &Cauldron {
        return &self.cauldron;
    }

    pub fn cauldron_mut(&mut self) -> &mut Cauldron {
        return &mut self.cauldron;
    }

    pub fn add_chip_to_cauldron(&mut self, chip: Chip) {
        self.cauldron.add_chip(chip);
        log::info!("{self} added {chip} to cauldron");
    }

    pub fn empty_cauldron(&mut self) {
        let mut chips = self.cauldron.remove_all();
        self.bag.append(&mut chips);
    }

    pub fn bag(&self) -> &Vec<Chip> {
        &self.bag
    }

    pub fn bag_mut(&mut self) -> &mut Vec<Chip> {
        &mut self.bag
    }

    pub fn add_chip_to_bag(&mut self, chip: Chip) {
        self.bag.push(chip);
    }

    pub fn flask(&self) -> bool {
        self.flask
    }

    pub fn use_flask(&mut self) {
        self.flask = false;
    }

    pub fn can_use_flask(&self) -> bool {
        if !self.flask {
            return false;
        }
        self.cauldron.last_chip().map(|c| c.color()) == Some(Color::White)
    }

    pub fn drop_position(&self) -> u8 {
        self.drop
    }

    pub fn rubies(&self) -> u8 {
        self.rubies
    }

    pub fn add_rubies(&mut self, cnt: u8) {
        self.rubies += cnt;
    }

    pub fn victory_points(&self) -> u16 {
        self.score
    }

    pub fn add_victory_points(&mut self, points: u8) {
        self.score = self.score + u16::from(points);
    }

    pub fn move_drop(&mut self) {
        self.drop += 1;
    }

    pub fn wants_to_continue_drawing(&self, game: &Game) -> bool {
        self.strategy.continue_drawing(game, &self)
    }

    pub fn wants_to_use_flask(&self, game: &Game) -> bool {
        self.strategy.spend_flask(game, &self)
    }

    pub fn wants_to_buy_instead_of_points(&self, game: &Game) -> bool {
        self.strategy.buy_instead_of_points(game, &self)
    }

    pub fn choose_chips_to_add_to_bag(
        &self,
        game: &Game,
        options: &Vec<Vec<Chip>>,
    ) -> Option<usize> {
        self.strategy
            .choose_chips_to_add_to_bag(game, &self, options)
    }

    pub fn choose_chips_to_add_to_cauldon(&self, chips: &Vec<Chip>) -> Option<usize> {
        self.strategy.choose_chips_to_add_to_cauldon(&self, chips)
    }
}
