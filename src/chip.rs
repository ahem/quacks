pub enum Color {
    White,
    Orange,
    Green,
    Blue,
    Red,
    Yellow,
    Purple,
    Black,
}

#[derive(Debug, Clone)]
pub enum Chip {
    White1,
    White2,
    White3,
    Orange1,
    Green1,
    Green2,
    Green4,
    Blue1,
    Blue2,
    Blue4,
    Red1,
    Red2,
    Red4,
    Yellow1,
    Yellow2,
    Yellow4,
    Purple1,
    Black1,
}

impl Chip {
    pub fn value(&self) -> u8 {
        return match self {
            Chip::White1
            | Chip::Orange1
            | Chip::Green1
            | Chip::Blue1
            | Chip::Red1
            | Chip::Black1
            | Chip::Yellow1
            | Chip::Purple1 => 1,
            Chip::White2 | Chip::Green2 | Chip::Blue2 | Chip::Red2 | Chip::Yellow2 => 2,
            Chip::White3 => 3,
            Chip::Green4 | Chip::Blue4 | Chip::Red4 | Chip::Yellow4 => 4,
        };
    }

    pub fn color(&self) -> Color {
        return match self {
            Chip::White1 | Chip::White2 | Chip::White3 => Color::White,
            Chip::Orange1 => Color::Orange,
            Chip::Green1 | Chip::Green2 | Chip::Green4 => Color::Green,
            Chip::Blue1 | Chip::Blue2 | Chip::Blue4 => Color::Blue,
            Chip::Red1 | Chip::Red2 | Chip::Red4 => Color::Red,
            Chip::Yellow1 | Chip::Yellow2 | Chip::Yellow4 => Color::Yellow,
            Chip::Purple1 => Color::Purple,
            Chip::Black1 => Color::Black,
        };
    }
}
