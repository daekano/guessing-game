use rand::Rng;
use std::{thread, time};

struct GameState {
    number: u32,
    guesses: u32,
    min_number: u32,
    max_number: u32,
    time_wait: time::Duration,
}

impl GameState {
    fn new(min: u32, max: u32) -> GameState {
        GameState {
            number: pick_number(min, max),
            guesses: 0,
            min_number: min,
            max_number: max,
            time_wait: time::Duration::from_millis(500),
        }
    }
}

fn main() {
    let min_number = 0;
    let max_number = 10;
    let mut game = new_game(min_number, max_number);
    loop {
        let result = run_game(&mut game);
        if result {
            game = new_game(min_number, max_number);
        }
    }
}

fn run_game(game: &mut GameState) -> bool {
    let guess = pick_number(game.min_number, game.max_number);
    let guessed = guess_number(&game, guess);
    game.guesses += 1;
    println!("guess: {}, guessed: {}", guess, guessed);
    if guessed {
        println!(
            "the computer guessed the number after {} guesses",
            game.guesses
        );
        return true;
    }
    thread::sleep(game.time_wait);
    return false;
}

fn new_game(min_number: u32, max_number: u32) -> GameState {
    let game = GameState::new(min_number, max_number);
    println!("new game, number: {}", game.number);
    return game;
}

fn pick_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

fn guess_number(game: &GameState, guess: u32) -> bool {
    game.number == guess
}
