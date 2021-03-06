use super::model;
use std::{thread, time};

const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 10;
const TIME_LOOP_WAIT: time::Duration = time::Duration::from_millis(500);

pub struct World {
    game: model::GameState,
    time_wait: time::Duration,
}

impl World {
    pub fn new() -> World {
        World {
            game: model::GameState::new(MIN_NUMBER, MAX_NUMBER),
            time_wait: TIME_LOOP_WAIT,
        }
    }
    pub fn start(&mut self) {
        println!("game started! number to guess: {}", self.game.number());
        loop {
            let events =
                model::GameState::run(&mut self.game, model::pick_number(MIN_NUMBER, MAX_NUMBER));
            for event in events {
                match event {
                    model::GameEvent::ClientGuessed(guess, guessed) => {
                        println!("guess: {}, guessed: {}", guess, guessed)
                    }
                    model::GameEvent::GameCompleted(guesses) => {
                        model::GameState::reset(&mut self.game);
                        println!("the computer guessed the number after {} guesses", guesses);
                        println!("the new number to guess is: {}", self.game.number());
                    }
                }
            }
            thread::sleep(self.time_wait);
        }
    }
}
