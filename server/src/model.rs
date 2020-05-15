use rand::Rng;

pub struct GameState {
    number: u32,
    guesses: u32,
    min_number: u32,
    max_number: u32,
}

impl GameState {
    pub fn number(&self) -> &u32 {
        return &self.number;
    }
}

#[derive(PartialEq)]
pub enum GameEvent {
    ClientGuessed(u32, bool),
    GameCompleted(u32),
}

impl GameState {
    pub fn new(min_number: u32, max_number: u32) -> GameState {
        GameState {
            number: pick_number(min_number, max_number),
            guesses: 0,
            min_number: min_number,
            max_number: max_number,
        }
    }
    pub fn reset(state: &mut GameState) {
        state.guesses = 0;
        state.number = pick_number(state.min_number, state.max_number);
    }
    pub fn run(state: &mut GameState, guess: u32) -> Vec<GameEvent> {
        let mut game_events: Vec<GameEvent> = Vec::new();
        let guessed = state.number == guess;
        state.guesses += 1;
        if guessed {
            game_events.push(GameEvent::GameCompleted(state.guesses));
        }
        game_events.push(GameEvent::ClientGuessed(guess, guessed));

        return game_events;
    }
}

pub fn pick_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_gamestate_reset() {
        let mut state = super::GameState {
            guesses: 10,
            number: 50,
            min_number: 1,
            max_number: 10,
        };

        super::GameState::reset(&mut state);

        assert_eq!(state.guesses, 0);
        assert!(state.number <= 10);
    }
    #[test]
    fn test_gamestate_run_completed() {
        let mut state = super::GameState {
            guesses: 0,
            number: 5,
            min_number: 1,
            max_number: 10,
        };

        let events = super::GameState::run(&mut state, 5);
        let expected_events: Vec<super::GameEvent> = vec![
            super::GameEvent::GameCompleted(1),
            super::GameEvent::ClientGuessed(5, true),
        ];
        assert!(events == expected_events);
    }

    #[test]
    fn test_gamestate_run() {
        let mut state = super::GameState {
            guesses: 0,
            number: 1,
            min_number: 1,
            max_number: 10,
        };

        let events = super::GameState::run(&mut state, 5);
        let expected_events: Vec<super::GameEvent> =
            vec![super::GameEvent::ClientGuessed(5, false)];
        assert!(events == expected_events);
    }
}
