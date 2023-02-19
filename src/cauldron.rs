use crate::chip::{Chip, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    pub coins: u8,
    pub points: u8,
    pub ruby: bool,
}

#[derive(Debug)]
pub struct Cauldron {
    limit: u8,
    position: u8,
    chips: Vec<Chip>,
}

impl Cauldron {
    pub fn new() -> Self {
        Self {
            position: 0,
            limit: 7,
            chips: vec![],
        }
    }

    pub fn is_full(&self) -> bool {
        usize::from(self.position) >= CAULDRON_FIELDS.len() - 1
    }

    pub fn increase_position(&mut self, n: u8) {
        self.position += n
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

    pub fn chips(&self) -> &Vec<Chip> {
        &self.chips
    }

    pub fn remove_last(&mut self) -> Option<Chip> {
        if let Some(chip) = self.chips.pop() {
            self.position -= chip.value();
            Some(chip)
        } else {
            None
        }
    }

    pub fn remove_all(&mut self) -> Vec<Chip> {
        let chips = self.chips.clone();
        self.chips.clear();
        self.position = 0;
        chips
    }

    pub fn number_of(&self, color: Color) -> u8 {
        self.chips
            .iter()
            .fold(0, |acc, c| if c.color() == color { acc + 1 } else { acc })
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
    Score { coins: 0, points: 0, ruby: false },
    Score { coins: 1, points: 0, ruby: false },
    Score { coins: 2, points: 0, ruby: false },
    Score { coins: 3, points: 0, ruby: false },
    Score { coins: 4, points: 0, ruby: false },
    Score { coins: 5, points: 0, ruby: true },
    Score { coins: 6, points: 1, ruby: false },
    Score { coins: 7, points: 1, ruby: false },
    Score { coins: 8, points: 1, ruby: false },
    Score { coins: 9, points: 1, ruby: true },
    Score { coins: 10, points: 2, ruby: false },
    Score { coins: 11, points: 2, ruby: false },
    Score { coins: 12, points: 2, ruby: false },
    Score { coins: 13, points: 2, ruby: true },
    Score { coins: 14, points: 3, ruby: false },
    Score { coins: 15, points: 3, ruby: false },
    Score { coins: 16, points: 3, ruby: false },
    Score { coins: 16, points: 4, ruby: false },
    Score { coins: 17, points: 4, ruby: false },
    Score { coins: 17, points: 4, ruby: true },
    Score { coins: 18, points: 4, ruby: false },
    Score { coins: 18, points: 5, ruby: false },
    Score { coins: 19, points: 5, ruby: false },
    Score { coins: 19, points: 5, ruby: true },
    Score { coins: 20, points: 5, ruby: false },
    Score { coins: 20, points: 6, ruby: false },
    Score { coins: 21, points: 6, ruby: false },
    Score { coins: 21, points: 6, ruby: true },
    Score { coins: 22, points: 7, ruby: false },
    Score { coins: 22, points: 7, ruby: true },
    Score { coins: 23, points: 7, ruby: false },
    Score { coins: 23, points: 8, ruby: false },
    Score { coins: 24, points: 8, ruby: false },
    Score { coins: 24, points: 8, ruby: true },
    Score { coins: 25, points: 9, ruby: false },
    Score { coins: 25, points: 9, ruby: true },
    Score { coins: 26, points: 9, ruby: false },
    Score { coins: 26, points: 10, ruby: false },
    Score { coins: 27, points: 10, ruby: false },
    Score { coins: 27, points: 10, ruby: true },
    Score { coins: 28, points: 11, ruby: false },
    Score { coins: 29, points: 11, ruby: false },
    Score { coins: 29, points: 12, ruby: false },
    Score { coins: 30, points: 12, ruby: false },
    Score { coins: 30, points: 12, ruby: true },
    Score { coins: 31, points: 12, ruby: false },
    Score { coins: 31, points: 13, ruby: false },
    Score { coins: 32, points: 13, ruby: false },
    Score { coins: 32, points: 13, ruby: true },
    Score { coins: 33, points: 14, ruby: false },
    Score { coins: 33, points: 14, ruby: true },
    Score { coins: 35, points: 15, ruby: false },
];
