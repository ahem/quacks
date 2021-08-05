mod bag;
mod chip;
mod potion_phase;

use chip::Chip;
use rand::thread_rng;
use rand::Rng;

fn draw(bag: &mut Vec<Chip>, rng: &mut rand::rngs::ThreadRng) -> Chip {
    let idx = rng.gen_range(0..bag.len());
    return bag.remove(idx);
}

fn main() {
    let mut rng = thread_rng();
    let mut bag = vec![
        Chip::White1,
        Chip::White1,
        Chip::White1,
        Chip::White1,
        Chip::White2,
        Chip::White2,
        Chip::White3,
        Chip::Orange1,
        Chip::Green1,
    ];
    let chip = draw(&mut bag, &mut rng);
    println!("Drawn chip: {:?}", chip);
    let chip = draw(&mut bag, &mut rng);
    println!("Drawn chip: {:?}", chip);
    let chip = draw(&mut bag, &mut rng);
    println!("Drawn chip: {:?}", chip);
}
