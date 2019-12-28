use crate::game::*;
use crate::players::Player;
use rand::prelude::*;

#[allow(dead_code)]
pub struct RandomPlayer {
    rng: ThreadRng,
}

#[allow(dead_code)]
impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        RandomPlayer { rng: thread_rng() }
    }
}

impl Player for RandomPlayer {
    fn name(&self) -> Option<String> {
        Some("Random IA".to_string())
    }

    fn next_movement(&self, game: &Game) -> i32 {
        let columns = game.get_board().get_unfilled_columns();
        let mut rng = self.rng;

        return *columns.choose(&mut rng).unwrap();
    }
}
