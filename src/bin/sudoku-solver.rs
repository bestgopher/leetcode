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
        if start.0 == 8 && start.1 == 8 && board[start.0][start.1] != '.' {
            return true;
        }

        let mut index = (start.0, start.1);
        while index.0 < 9 && index.1 < 9 {
            if board[index.0][index.1] == '.' {
                break;
            }
            index.1 += 1;
            if index.1 == 9 {
                index.0 += 1;
                index.1 = 0;
            }
        }

        if index.0 == 9 || index.1 == 9 {
            return true;
        }

        'Loop: for value in '1'..='9' {
            // 检查行和列
            for x in 0..9 {
                if board[index.0][x] == value {
                    continue 'Loop;
                }

                if board[x][index.1] == value {
                    continue 'Loop;
                }
            }

            // 检查四周
            for i in index.0 / 3 * 3..index.0 / 3 * 3 + 3 {
                for j in index.1 / 3 * 3..index.1 / 3 * 3 + 3 {
                    if board[i][j] == value {
                        continue 'Loop;
                    }
                }
            }

            board[index.0][index.1] = value;

            if !Solution::rec(board, index) {
                board[index.0][index.1] = '.';
            } else {
                return true;
            }
        }

        false
    }
}
