use rand::Rng;

pub const EVENT_GAME_COMPLETE: &str = "game_complete";
pub const EVENT_GAME_CONTINUING: &str = "game_continuing";

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

pub struct GameResult {
    messages: Vec<String>,
    event: String,
}

impl GameResult {
    pub fn new() -> GameResult {
        GameResult {
            messages: Vec::new(),
            event: "".to_owned(),
        }
    }
    pub fn messages(&self) -> &Vec<String> {
        return &self.messages;
    }
    pub fn event(&self) -> &String {
        return &self.event;
    }
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
        println!("new number: {}", self.number);
    }
    pub fn run(&mut self) -> GameResult {
        let mut game_result = GameResult::new();
        let guess = pick_number(self.min_number, self.max_number);
        let guessed = guess_number(&self, guess);
        self.guesses += 1;
        game_result
            .messages
            .push(format!("guess: {}, guessed: {}", guess, guessed));
        if guessed {
            game_result.messages.push(format!(
                "the computer guessed the number after {} guesses",
                self.guesses
            ));
            game_result.event = EVENT_GAME_COMPLETE.to_owned();
        } else {
            game_result.event = EVENT_GAME_CONTINUING.to_owned();
        }

        return game_result;
    }
}

fn guess_number(game: &GameState, guess: u32) -> bool {
    game.number == guess
}

fn pick_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
