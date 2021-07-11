fn main() {}

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.as_bytes();
        let mut board = board;

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] as u8 == word[0] {
                    let x = board[i][j];
                    board[i][j] = '0';
                    if Self::scan(&mut board[..], &word[1..], (i, j)) {
                        return true;
                    }
                    board[i][j] = x;
                }
            }
        }

        false
    }

    fn scan(board: &mut [Vec<char>], word: &[u8], index: (usize, usize)) -> bool {
        if word.len() == 0 {
            return true;
        }

        if index.0 > 0 {
            let x = board[index.0 - 1][index.1];
            board[index.0 - 1][index.1] = '0';
            if x != '0'
                && x as u8 == word[0]
                && Self::scan(board, &word[1..], (index.0 - 1, index.1))
            {
                return true;
            }
            board[index.0 - 1][index.1] = x;
        }

        if index.0 < board.len() - 1 {
            let x = board[index.0 + 1][index.1];
            board[index.0 + 1][index.1] = '0';
            if x != '0'
                && x as u8 == word[0]
                && Self::scan(board, &word[1..], (index.0 + 1, index.1))
            {
                return true;
            }
            board[index.0 + 1][index.1] = x;
        }

        if index.1 > 0 {
            let x = board[index.0][index.1 - 1];
            board[index.0][index.1 - 1] = '0';
            if x != '0'
                && x as u8 == word[0]
                && Self::scan(board, &word[1..], (index.0, index.1 - 1))
            {
                return true;
            }
            board[index.0][index.1 - 1] = x;
        }

        if index.1 < board[0].len() - 1 {
            let x = board[index.0][index.1 + 1];
            board[index.0][index.1 + 1] = '0';
            if x != '0'
                && x as u8 == word[0]
                && Self::scan(board, &word[1..], (index.0, index.1 + 1))
            {
                return true;
            }
            board[index.0][index.1 + 1] = x;
        }

        false
    }
}
