use crate::thinking::{GameResult, Hand, Thinking};
use std::fmt;

#[derive(Debug)]
pub struct Player<T> {
    name: String,
    strategy: T,
    wincount: u32,
    losecount: u32,
    evencount: u32,
    gamecount: u32,
}

impl<T: Thinking> Player<T> {
    pub fn new(name: String, strategy: T) -> Self {
        Self {
            name,
            strategy,
            wincount: 0,
            losecount: 0,
            evencount: 0,
            gamecount: 0,
        }
    }

    pub fn next_hand(&mut self) -> Hand {
        self.strategy.next_hand()
    }

    pub fn win(&mut self) {
        self.strategy.study(GameResult::Win);
        self.wincount += 1;
        self.gamecount += 1;
    }

    pub fn lose(&mut self) {
        self.strategy.study(GameResult::Lose);
        self.losecount += 1;
        self.gamecount += 1;
    }

    pub fn even(&mut self) {
        self.strategy.study(GameResult::Even);
        self.evencount += 1;
        self.gamecount += 1;
    }
}

impl<T: Thinking> fmt::Display for Player<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "{} : {} games, {} win {} lose {} even",
            self.name, self.gamecount, self.wincount, self.losecount, self.evencount
        )
    }
}
