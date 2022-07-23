#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(92, Solution::total_n_queens(8));
}

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut nums = vec![vec![b'.'; n as usize]; n as usize];
        let mut result = 0;
        Self::find(&mut nums, n, &mut result);
        result
    }

    fn find(nums: &mut Vec<Vec<u8>>, n: i32, result: &mut i32) {
        if n == 0 {
            *result += 1;
            return;
        }

        let len = nums.len();

        for j in 0..len {
            if !Self::check(nums, len - n as usize, j) {
                continue;
            }

            nums[len - n as usize][j] = b'Q';
            Self::find(nums, n - 1, result);
            nums[len - n as usize][j] = b'.';
        }
    }

    /// 检查(i, j)在nums中是否满足不受攻击
    fn check(nums: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
        let mut index = 1;
        let len = nums.len();

        while i >= index {
            // 判断与(i, j)同列的是否为皇后
            if nums[i - index][j] == b'Q' {
                return false;
            }

            // 判断(i, j)的左上斜线是否为皇后
            if j >= index && nums[i - index][j - index] == b'Q' {
                return false;
            }

            // 判断(i, j)的右上斜线是否为皇后
            if j + index < len && nums[i - index][j + index] == b'Q' {
                return false;
            }
            index += 1;
        }
        true
    }
}
