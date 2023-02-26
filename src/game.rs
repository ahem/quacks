use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::bonus_die::{BonusDie, BonusDieResult};
use crate::chip::Chip;
use crate::player::Player;
use crate::rules::RuleSet;

pub trait Rule {
    fn chip_drawn(&self, player: Rc<RefCell<Player>>);
    fn cauldron_finished(&self, player: Rc<RefCell<Player>>);
    fn black_chip(&self, player: Rc<RefCell<Player>>, game: &Game);
    fn green_chip(&self, player: Rc<RefCell<Player>>);
    fn purple_chip(&self, player: Rc<RefCell<Player>>);
}

pub struct Game {
    players: Vec<Rc<RefCell<Player>>>,
    bonus_die: BonusDie,
    pub rules: RuleSet,
    pub turn: u8,
    pub rubies_to_fill_flask: u8,
    pub rubies_to_move_drop: u8,
}

impl Game {
    pub fn new(players: Vec<Player>, rules: RuleSet, bonus_die: BonusDie) -> Self {
        let players = players
            .into_iter()
            .map(|p| Rc::new(RefCell::new(p)))
            .collect();
        Self {
            players,
            rules,
            bonus_die,
            turn: 1,
            rubies_to_fill_flask: 2,
            rubies_to_move_drop: 2,
        }
    }

    pub fn players(&self) -> Vec<Ref<Player>> {
        self.players.iter().map(|p| p.borrow()).collect()
    }

    pub fn player_position(&self, player: &Rc<RefCell<Player>>) -> usize {
        self.players
            .iter()
            .position(|p| Rc::ptr_eq(p, player))
            .expect("player doesn't exist in game")
    }

    fn calculate_rat_tails(&self, player: &Rc<RefCell<Player>>) -> u8 {
        const RAT_TAIL_BEFORE: [u16; 23] = [
            2, 5, 8, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49,
        ];
        let leading_player_poaition = self
            .players()
            .iter()
            .map(|p| p.victory_points())
            .max()
            .unwrap();

        let player_position = player.borrow().victory_points();

        let mut rat_tails = 0;
        if player_position < leading_player_poaition {
            for pos in player_position + 1..=leading_player_poaition {
                if RAT_TAIL_BEFORE.contains(&(pos % 50)) {
                    rat_tails += 1;
                }
            }
        }
        rat_tails
    }
}

fn fill_cauldron_phase(game: &Game) {
    let mut active_players = game.players.clone();

    for player in &active_players {
        let drop_position = player.borrow().drop_position();
        let rat_tails = game.calculate_rat_tails(player);
        player
            .borrow_mut()
            .cauldron_mut()
            .increase_position(drop_position + rat_tails);

        log::debug!(
            "{player} starting draw [{drop_position} + {rat_tails}]: {bag:?}",
            player = player.borrow(),
            bag = player.borrow().bag(),
        );
    }

    while !active_players.is_empty() {
        for player in &active_players {
            // draw chip
            let chip = player.borrow_mut().draw();
            player.borrow_mut().add_chip_to_cauldron(chip);
            game.rules.apply_chip_drawn_rules(player.clone(), game);

            if player.borrow().cauldron().is_exploded() {
                continue;
            }

            if player.borrow().can_use_flask() && player.borrow().wants_to_use_flask(game) {
                let mut player = player.borrow_mut();
                let chip = player.cauldron_mut().remove_last().unwrap();
                player.bag_mut().push(chip);
                player.use_flask();
            }
        }

        active_players = active_players
            .iter()
            .cloned()
            .filter(|player| {
                !player.borrow().cauldron().is_exploded()
                    && !player.borrow().cauldron().is_full()
                    && player.borrow().wants_to_continue_drawing(game)
            })
            .collect();
    }

    for player in game.players.iter().cloned() {
        game.rules.apply_cauldron_finished_rules(&player);
        let player = player.borrow();
        let cauldron = player.cauldron();
        let score = cauldron.score();
        if cauldron.is_exploded() {
            log::info!("{player} exploded!!!");
        }
        log::debug!("{player} finished drawing: {cauldron:#?}");
        log::info!("{player} {score:?}");
    }
}

fn bonus_die_phase(game: &Game) {
    let players: Vec<_> = game
        .players
        .iter()
        .filter(|p| !p.borrow().cauldron().is_exploded())
        .collect();

    let max_score =
        if let Some(max_score) = players.iter().map(|p| p.borrow().cauldron().score()).max() {
            max_score
        } else {
            return;
        };

    let players = players
        .iter()
        .filter(|p| p.borrow().cauldron().score() == max_score);

    for player in players {
        log::info!("{player} rolls bonus die", player = player.borrow());
        let roll = game.bonus_die.roll();
        match roll {
            BonusDieResult::OrangeChip => {
                player.borrow_mut().bag_mut().push(Chip::Orange1);
            }
            BonusDieResult::VictoryPoint(points) => {
                player.borrow_mut().add_victory_points(points);
            }
            BonusDieResult::Ruby => {
                player.borrow_mut().add_rubies(1);
            }
            BonusDieResult::Drop => {
                player.borrow_mut().move_drop();
            }
        }
        game.rules.apply_bonus_die_rolled_rules(player, roll);
    }
}

fn buy_chips_phase(game: &Game, player: &Rc<RefCell<Player>>, coins: u8) {
    let options = game.rules.purchase_options(game, coins);
    let choice = player.borrow().choose_chips_to_add_to_bag(game, &options);
    if let Some(idx) = choice {
        if let Some(chips) = options.get(idx) {
            for chip in chips {
                player.borrow_mut().add_chip_to_bag(*chip);
                log::info!("{player} bought {chip}", player = player.borrow());
            }
        }
    }
}

fn spend_rubies_phase(game: &Game, player: &Rc<RefCell<Player>>) {
    if !player.borrow().flask() && player.borrow().rubies() >= game.rubies_to_fill_flask {
        if player.borrow().wants_to_pay_rubies_to_fill_flask(game) {
            let mut player = player.borrow_mut();
            player.subtract_rubies(game.rubies_to_fill_flask);
            player.fill_flask();
            log::info!(
                "{player} fills flask for {price} rubies",
                price = game.rubies_to_fill_flask
            );
        }
    }

    let mut drop_moves = 0;
    while player.borrow().rubies() >= game.rubies_to_move_drop {
        if player.borrow().wants_to_pay_rubies_to_move_drop(game) {
            let mut player = player.borrow_mut();
            player.subtract_rubies(game.rubies_to_move_drop);
            player.move_drop();
            drop_moves += 1;
        } else {
            break;
        }
    }
    if drop_moves > 0 {
        log::info!(
            "{player} moves drop {drop_moves} places for {price} rubies",
            player = player.borrow(),
            price = game.rubies_to_move_drop * drop_moves
        );
    }
}

fn round(game: &mut Game) {
    if game.turn == 6 {
        for player in game.players.iter() {
            player.borrow_mut().bag_mut().push(Chip::White1);
        }
    }

    // TODO: draw card

    fill_cauldron_phase(game);

    bonus_die_phase(game);

    for player in game.players.iter() {
        game.rules.apply_black_chip_rules(&player, game);
        game.rules.apply_green_chip_rules(&player);
        game.rules.apply_purple_chip_rules(&player);
    }

    for player in game.players.iter() {
        let score = player.borrow().cauldron().score();
        if score.ruby {
            player.borrow_mut().add_rubies(1);
        }
        if !player.borrow().cauldron().is_exploded() {
            // TODO: in round 9 coins become points
            buy_chips_phase(&game, player, score.coins);
            player.borrow_mut().add_victory_points(score.points);
        } else {
            // TODO: in round 9 always choose score
            if player.borrow().wants_to_buy_instead_of_points(&game) {
                buy_chips_phase(&game, player, score.coins);
            } else {
                player.borrow_mut().add_victory_points(score.points);
            }
        }
    }

    for player in game.players.iter() {
        // TODO: in round 9 rubies bocomes points
        spend_rubies_phase(&game, player);
    }

    for player in game.players.iter() {
        player.borrow_mut().empty_cauldron();
    }
}

pub fn run(game: &mut Game) {
    for turn in 1..=9 {
        log::info!("start of round {turn}");
        game.turn = turn;
        round(game);
        log::info!("after round {turn}");
        for player in game.players() {
            log::info!("{player} score: {score}", score = player.victory_points());
        }
    }

    // TODO: calculate final score

    for player in game.players.iter() {
        let player = player.borrow();
        log::debug!("{player:#?}");
    }
}
