fn main() {}

struct Solution;

impl Solution {
    /// 通过战舰头部的个数来判断
    /// 当'X'的上边和左边是'X'，则说明这是舰身
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut num = 0;

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' {
                    if i > 0 && board[i - 1][j] == 'X' {
                        continue;
                    }

                    if j > 0 && board[i][j - 1] == 'X' {
                        continue;
                    }
                    num += 1;
                }
            }
        }

        num
    }
}
