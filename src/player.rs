use crate::chip::Chip;

#[derive(Debug)]
pub struct Player {
    score: u16,
    chips: Vec<Chip>,
    flask: bool,
    drop: u8,
}

impl Player {
    pub fn new() -> Self {
        Self {
            score: 0,
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

    pub fn drop_position(&self) -> u8 {
        self.drop
    }
}
