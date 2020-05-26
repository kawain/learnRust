mod player;
mod thinking;

use crate::player::Player;
use crate::thinking::{Hand, Way1, Way2};

fn main() {
    let mut player1 = Player::new("Taro".to_string(), Way1::new());
    let mut player2 = Player::new("Hana".to_string(), Way2::new());

    for i in 1..=1000 {
        let hand1: Hand = player1.next_hand();
        let hand2: Hand = player2.next_hand();
        match hand1 {
            Hand::Rock => {
                if hand2 == Hand::Scissors {
                    // 1の勝ち
                    println!("{:>04} Winner:{}", i, player1);
                    player1.win();
                    player2.lose();
                } else if hand2 == Hand::Paper {
                    // 1の負け
                    println!("{:>04} Winner:{}", i, player2);
                    player1.lose();
                    player2.win();
                } else {
                    // 引き分け
                    println!("{:>04} Even...", i);
                    player1.even();
                    player2.even();
                }
            }
            Hand::Scissors => {
                if hand2 == Hand::Paper {
                    // 1の勝ち
                    println!("{:>04} Winner:{}", i, player1);
                    player1.win();
                    player2.lose();
                } else if hand2 == Hand::Rock {
                    // 1の負け
                    println!("{:>04} Winner:{}", i, player2);
                    player1.lose();
                    player2.win();
                } else {
                    // 引き分け
                    println!("{:>04} Even...", i);
                    player1.even();
                    player2.even();
                }
            }
            Hand::Paper => {
                if hand2 == Hand::Rock {
                    // 1の勝ち
                    println!("{:>04} Winner:{}", i, player1);
                    player1.win();
                    player2.lose();
                } else if hand2 == Hand::Scissors {
                    // 1の負け
                    println!("{:>04} Winner:{}", i, player2);
                    player1.lose();
                    player2.win();
                } else {
                    // 引き分け
                    println!("{:>04} Even...", i);
                    player1.even();
                    player2.even();
                }
            }
        }
    }

    println!("\nTotal Result:");
    println!("{}", player1);
    println!("{}", player2);
}
