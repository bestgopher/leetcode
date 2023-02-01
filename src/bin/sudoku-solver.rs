#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    let mut v = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    Solution::solve_sudoku(&mut v);

    for i in v {
        println!("{i:?}");
    }
}

struct Solution;

impl Solution {
    /// 暴力解法，挨个枚举就完事
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::rec(board, (0, 0));
    }

    pub fn rec(board: &mut Vec<Vec<char>>, start: (usize, usize)) -> bool {
        if start.0 > 8 || start.1 > 8 {
            return false;
        }

        if start.0 == 8 && start.1 == 8 && board[start.0][start.1] != '.' {
            return true;
        }

        // 下一个坐标
        let mut index = if start.1 == 8 {
            (start.0 + 1, 0)
        } else {
            (start.0, start.1 + 1)
        };

        if board[start.0][start.1] != '.' {
            return Self::rec(board, index);
        }

        'Loop: for value in '1'..='9' {
            // 检查行和列
            for x in 0..9 {
                if board[start.0][x] == value {
                    continue 'Loop;
                }

                if board[x][start.1] == value {
                    continue 'Loop;
                }
            }

            // 检查四周
            for i in start.0 / 3 * 3..start.0 / 3 * 3 + 3 {
                for j in start.1 / 3 * 3..start.1 / 3 * 3 + 3 {
                    if board[i][j] == value {
                        continue 'Loop;
                    }
                }
            }

            board[start.0][start.1] = value;

            if !Solution::rec(board, index) {
                board[start.0][start.1] = '.';
            } else {
                return true;
            }
        }

        false
    }
}
