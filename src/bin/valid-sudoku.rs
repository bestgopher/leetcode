#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = [[0; 10]; 9];
        let mut column = [[0; 10]; 9];
        let mut boxes = [[[0; 10]; 3]; 3];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                let x: usize = board[i][j].to_string().parse().unwrap();
                if row[i][x] != 0 {
                    return false;
                } else {
                    row[i][x] = 1;
                }

                if column[j][x] != 0 {
                    return false;
                } else {
                    column[j][x] = 1;
                }

                if boxes[i / 3][j / 3][x] != 0 {
                    return false;
                } else {
                    boxes[i / 3][j / 3][x] = 1;
                }
            }
        }

        true
    }
}
