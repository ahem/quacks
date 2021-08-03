use crate::chip::{Chip, Color};
use rand::prelude::*;

pub enum Action {
    Draw,
    Stop,
    Place,
    PlaceAndSelect(Chip),
}

struct State {
    chip: Option<Chip>,
    bag: Vec<Chip>,
    potion: Vec<Chip>,
}

fn collect_possible_actions(state: &State, rng: &mut rand::rngs::ThreadRng) -> Vec<Action> {
    // if state.potion.is_exploded() { vec![Action::Stop]; };
    return match state.chip {
        None => vec![Action::Stop, Action::Draw],
        Some(chip) => match chip.color() {
            Color::White
            | Color::Orange
            | Color::Red
            | Color::Green
            | Color::Black
            | Color::Yellow
            | Color::Purple => vec![Action::Place],
            Color::Blue => {
                let val = chip.value();
                let options: Vec<Action> = state
                    .bag
                    .choose_multiple(rng, val.into())
                    .map(|x| Action::PlaceAndSelect(x.clone()))
                    .collect();
            }
        },
    };
}

pub fn run(
    bag: &Vec<Chip>,
    rng: &mut rand::rngs::ThreadRng,
    choose_action: fn(Vec<Action>) -> Action,
) -> Vec<Chip> {
    let state = State {
        chip: None,
        bag: bag.clone(),
        potion: Vec::new(),
    };

    let possible_actions = collect_possible_actions(&state, &mut rng);
    let action = choose_action(possible_actions);

    return state.potion;
}
