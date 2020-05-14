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
    pub fn reset(&mut self) {
        self.guesses = 0;
        self.number = pick_number(self.min_number, self.max_number);
    }
    pub fn run(&mut self) -> Vec<GameEvent> {
        let mut game_events: Vec<GameEvent> = Vec::new();
        let guess = pick_number(self.min_number, self.max_number);
        let guessed = guess_number(&self, guess);
        self.guesses += 1;
        game_events.push(GameEvent::ClientGuessed(guess, guessed));
        if guessed {
            game_events.push(GameEvent::GameCompleted(self.guesses));
        }

        return game_events;
    }
}

fn guess_number(game: &GameState, guess: u32) -> bool {
    game.number == guess
}

fn pick_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
