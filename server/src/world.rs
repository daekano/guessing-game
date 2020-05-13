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
            thread::sleep(self.time_wait);
            let game_result = self.game.run();
            for message in game_result.messages() {
                println!("{}", message);
            }
            match game_result.event().as_str() {
                model::EVENT_GAME_COMPLETE => self.game.reset(),
                _ => continue,
            };
        }
    }
}
