#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let (mut r, mut j) = (0, 0);

        for i in 0..player1.len() {
            if i == 1 {
                if player1[0] == 10 {
                    r += player1[i];
                }
                if player2[0] == 10 {
                    j += player2[i];
                }
            } else if i > 1 {
                if player1[i - 1] == 10 || player1[i - 2] == 10 {
                    r += player1[i];
                }
                if player2[i - 1] == 10 || player2[i - 2] == 10 {
                    j += player2[i];
                }
            }

            r += player1[i];
            j += player2[i];
        }

        if r > j {
            1
        } else if j > r {
            2
        } else {
            0
        }
    }
}
