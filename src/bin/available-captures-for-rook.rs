#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut index = (0usize, 0usize);
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'R' {
                    index = (i, j);
                    break;
                }
            }
        }
        let mut result = 0;

        for i in (0..index.0).rev() {
            match board[i][index.1] {
                'B' => break,
                'p' => {
                    result += 1;
                    break;
                }
                _ => {}
            }
        }

        for i in (index.0..board.len()) {
            match board[i][index.1] {
                'B' => break,
                'p' => {
                    result += 1;
                    break;
                }
                _ => {}
            }
        }

        for i in (0..index.1).rev() {
            match board[index.0][i] {
                'B' => break,
                'p' => {
                    result += 1;
                    break;
                }
                _ => {}
            }
        }

        for i in (index.1..board[0].len()) {
            match board[index.0][i] {
                'B' => break,
                'p' => {
                    result += 1;
                    break;
                }
                _ => {}
            }
        }

        result
    }
}
