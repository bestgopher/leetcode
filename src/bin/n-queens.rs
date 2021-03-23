fn main() {}

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut r = Vec::new();
        for i in 0..n as usize {
            let mut v = vec![vec![b'.'; n as usize]; n as usize];
            v[0][i] = b'Q';
            if Self::queen(&mut v, 1) {
                r.push(v.into_iter().map(|x| String::from_utf8(x).unwrap()).collect());
            }
        }
        r
    }

    fn queen(nums: &mut Vec<Vec<u8>>, n: i32) -> bool {
        if n as usize > nums.len() {
            return true;
        }
        for i in 0..nums.len() {
            if Self::
        }
    }

    // 检查坐标(i,j)是否可以放置
    fn check(nums: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
        let (n1, n2) = (nums[i].len(), nums.len());
        for index in 0..n1 {
            // 检查第i行
            if nums[i][index] == b'Q' {
                return false;
            }
        }

        for index in 0..n2 {
            // 检查第i行
            if nums[index][j] == b'Q' {
                return false;
            }
        }

        // 检查主对角线
        let mut offset = 1;
        while i - offset >= 0 && j - offset >= 0 {
            if nums[i - offset][j - offset] == b'Q' {
                return false;
            }
            offset += 1;
        }

        let mut offset = 1;
        while i + offset < n1 && j + offset < n2 {
            if nums[i + offset][i + offset] == b'Q' {
                return false;
            }
            offset += 1;
        }

        // 检查副对角线
        let mut offset = 1;
        while i + offset < n1 && j - offset >= 0 {
            if nums[i + offset][j - offset] == b'Q' {
                return false;
            }
            offset += 1;
        }

        let mut offset = 1;
        while i - offset >= 0 && j + offset < n2 {
            if nums[i - offset][i + offset] == b'Q' {
                return false;
            }
            offset += 1;
        }

        true
    }
}
