use crate::chip::{Chip, Color};

#[derive(Debug, Clone, Copy)]
pub struct Score {
    value: u8,
    points: u8,
    ruby: bool,
}

#[derive(Debug)]
pub struct Cauldron {
    limit: u8,
    position: u8,
    chips: Vec<Chip>,
}

impl Cauldron {
    pub fn new(start_position: u8) -> Self {
        Self {
            position: start_position,
            limit: 7,
            chips: vec![],
        }
    }

    pub fn is_full(&self) -> bool {
        usize::from(self.position) >= CAULDRON_FIELDS.len() - 1
    }

    pub fn add_chip(&mut self, chip: Chip) {
        if self.is_full() {
            panic!("cannot put chips in full cauldron");
        }
        if self.is_exploded() {
            panic!("cannot put chips in exploded cauldron");
        }
        self.chips.push(chip);
        self.position += chip.value();
    }

    pub fn last_chip(&self) -> Option<Chip> {
        self.chips.last().cloned()
    }

    pub fn total_value_of(&self, color: Color) -> u8 {
        self.chips.iter().fold(0, |acc, c| {
            if c.color() == color {
                acc + c.value()
            } else {
                acc
            }
        })
    }

    pub fn is_exploded(&self) -> bool {
        self.total_value_of(Color::White) > self.limit
    }

    pub fn chance_to_explode(&self, bag: &Vec<Chip>) -> f32 {
        let current_value = self.total_value_of(Color::White);
        let bad_chips = bag.iter().fold(0u8, |acc, c| {
            if c.color() == Color::White && current_value + c.value() > self.limit {
                acc + 1
            } else {
                acc
            }
        });
        (bad_chips as f32) / (bag.len() as f32)
    }

    pub fn score(&self) -> Score {
        let pos = usize::min(usize::from(self.position), CAULDRON_FIELDS.len() - 1);
        CAULDRON_FIELDS[pos].to_owned()
    }
}

#[rustfmt::skip]
const CAULDRON_FIELDS: [Score; 52] = [
    Score { value: 0, points: 0, ruby: false },
    Score { value: 1, points: 0, ruby: false },
    Score { value: 2, points: 0, ruby: false },
    Score { value: 3, points: 0, ruby: false },
    Score { value: 4, points: 0, ruby: false },
    Score { value: 5, points: 0, ruby: true },
    Score { value: 6, points: 1, ruby: false },
    Score { value: 7, points: 1, ruby: false },
    Score { value: 8, points: 1, ruby: false },
    Score { value: 9, points: 1, ruby: true },
    Score { value: 10, points: 2, ruby: false },
    Score { value: 11, points: 2, ruby: false },
    Score { value: 12, points: 2, ruby: false },
    Score { value: 13, points: 2, ruby: true },
    Score { value: 14, points: 3, ruby: false },
    Score { value: 15, points: 3, ruby: false },
    Score { value: 16, points: 3, ruby: false },
    Score { value: 16, points: 4, ruby: false },
    Score { value: 17, points: 4, ruby: false },
    Score { value: 17, points: 4, ruby: true },
    Score { value: 18, points: 4, ruby: false },
    Score { value: 18, points: 5, ruby: false },
    Score { value: 19, points: 5, ruby: false },
    Score { value: 19, points: 5, ruby: true },
    Score { value: 20, points: 5, ruby: false },
    Score { value: 20, points: 6, ruby: false },
    Score { value: 21, points: 6, ruby: false },
    Score { value: 21, points: 6, ruby: true },
    Score { value: 22, points: 7, ruby: false },
    Score { value: 22, points: 7, ruby: true },
    Score { value: 23, points: 7, ruby: false },
    Score { value: 23, points: 8, ruby: false },
    Score { value: 24, points: 8, ruby: false },
    Score { value: 24, points: 8, ruby: true },
    Score { value: 25, points: 9, ruby: false },
    Score { value: 25, points: 9, ruby: true },
    Score { value: 26, points: 9, ruby: false },
    Score { value: 26, points: 10, ruby: false },
    Score { value: 27, points: 10, ruby: false },
    Score { value: 27, points: 10, ruby: true },
    Score { value: 28, points: 11, ruby: false },
    Score { value: 29, points: 11, ruby: false },
    Score { value: 29, points: 12, ruby: false },
    Score { value: 30, points: 12, ruby: false },
    Score { value: 30, points: 12, ruby: true },
    Score { value: 31, points: 12, ruby: false },
    Score { value: 31, points: 13, ruby: false },
    Score { value: 32, points: 13, ruby: false },
    Score { value: 32, points: 13, ruby: true },
    Score { value: 33, points: 14, ruby: false },
    Score { value: 33, points: 14, ruby: true },
    Score { value: 35, points: 15, ruby: false },
];
