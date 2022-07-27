#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == chars[0] {
                    let x = board[i][j];
                    board[i][j] = '0';
                    if Self::scan(i, j, &mut board[..], &chars[1..]) {
                        return true;
                    }

                    board[i][j] = x;
                }
            }
        }

        false
    }

    fn scan(i: usize, j: usize, board: &mut [Vec<char>], word: &[char]) -> bool {
        if word.is_empty() {
            return true;
        }

        if i > 0 {
            if board[i - 1][j] == word[0] {
                let x = board[i - 1][j];
                board[i - 1][j] = '0';
                if Self::scan(i - 1, j, board, &word[1..]) {
                    return true;
                }
                board[i - 1][j] = x;
            }
        }

        if j > 0 {
            if board[i][j - 1] == word[0] {
                let x = board[i][j - 1];
                board[i][j - 1] = '0';
                if Self::scan(i, j - 1, board, &word[1..]) {
                    return true;
                }
                board[i][j - 1] = x;
            }
        }

        if i < board.len() - 1 {
            if board[i + 1][j] == word[0] {
                let x = board[i + 1][j];
                board[i + 1][j] = '0';
                if Self::scan(i + 1, j, board, &word[1..]) {
                    return true;
                }
                board[i + 1][j] = x;
            }
        }

        if i <= board.len() - 1 && j < board[i].len() - 1 {
            if board[i][j + 1] == word[0] {
                let x = board[i][j + 1];
                board[i][j + 1] = '0';
                if Self::scan(i, j + 1, board, &word[1..]) {
                    return true;
                }
                board[i][j + 1] = x;
            }
        }

        false
    }
}
