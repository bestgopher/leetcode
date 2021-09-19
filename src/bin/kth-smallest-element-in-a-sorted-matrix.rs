fn main() {}

struct Solution;

impl Solution {
    /// 二分查找法？
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (mut left, mut right) = (matrix[0][0], matrix[matrix.len() - 1][matrix.len() - 1]);

        while left < right {
            let mid = left + (right - left) / 2;
            let num = Self::get_number(&matrix[..], mid);
            if num >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    /// 求个数
    fn get_number(matrix: &[Vec<i32>], mid: i32) -> i32 {
        let (mut i, mut j) = (matrix.len() as i32 - 1, 0i32);
        let mut num = 0;

        while i >= 0 && j < matrix.len() as i32 {
            if matrix[i as usize][j as usize] <= mid {
                num += i as i32 + 1;
                j += 1;
            } else {
                i -= 1;
            }
        }

        num
    }
}
