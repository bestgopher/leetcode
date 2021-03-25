fn main() {}

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut r = Vec::<Vec<String>>::new();
        let mut nums = vec![vec![b'.'; n as usize]; n as usize];
        Self::queen(&mut nums, n, &mut r);
        r
    }

    fn queen(nums: &mut Vec<Vec<u8>>, n: i32, r: &mut Vec<Vec<String>>) {
        if n == 0 {
            let mut x = Vec::with_capacity(nums.len());

            for i in nums.iter() {
                x.push(String::from_utf8(i.clone()).unwrap());
            }
            r.push(x);
            return;
        }
        let len = nums.len();

        for i in 0..len {
            // 如果当前点不满足条件，检查下一个点
            if !Self::check(nums, len - n as usize, i) { continue; }

            nums[len - n as usize][i] = b'Q';
            Self::queen(nums, n - 1, r);
            nums[len - n as usize][i] = b'.';
        }
    }

    // 检查坐标(i,j)是否可以放置
    fn check(nums: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
        let n2 = nums.len();

        for index in 0..i {
            // 检查同列
            if nums[index][j] == b'Q' {
                return false;
            }
        }

        // 检查主对角线上方
        let mut offset = 1;
        while i >= offset && j >= offset {
            if nums[i - offset][j - offset] == b'Q' {
                return false;
            }
            offset += 1;
        }
        // 检查副对角线上方
        let mut offset = 1;
        while i >= offset && j + offset < n2 {
            if nums[i - offset][j + offset] == b'Q' {
                return false;
            }
            offset += 1;
        }

        true
    }
}