use rand::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

const HAND_ARR: [Hand; 3] = [Hand::Rock, Hand::Paper, Hand::Scissors];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameResult {
    Win,
    Lose,
    Even,
}

pub trait Thinking {
    fn next_hand(&mut self) -> Hand;
    fn study(&mut self, win: GameResult);
}

#[derive(Debug, Copy, Clone)]
pub struct Way1 {
    prev_hand: Hand,
}

impl Way1 {
    pub fn new() -> Self {
        Self {
            prev_hand: Hand::Rock,
        }
    }
}

impl Thinking for Way1 {
    fn next_hand(&mut self) -> Hand {
        self.prev_hand
    }

    fn study(&mut self, win: GameResult) {
        match win {
            // 負け、引き分けはランダム
            GameResult::Lose | GameResult::Even => {
                let mut rng = thread_rng();
                let number = rng.gen_range(0, 2);
                self.prev_hand = HAND_ARR[number as usize];
            }
            // 勝ったら何もしない。前と同じ手
            GameResult::Win => (),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Way2 {
    hand: Hand,
}

impl Way2 {
    pub fn new() -> Self {
        Self { hand: Hand::Rock }
    }
}

impl Thinking for Way2 {
    fn next_hand(&mut self) -> Hand {
        self.hand
    }

    fn study(&mut self, _win: GameResult) {
        // 勝ちに関係なく以下の手順
        match self.hand {
            Hand::Rock => self.hand = Hand::Paper,
            Hand::Paper => self.hand = Hand::Scissors,
            Hand::Scissors => self.hand = Hand::Rock,
        }
    }
}
