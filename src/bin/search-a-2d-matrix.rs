#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    // 二叉搜索
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix[0].len() - 1;
        for i in 0..matrix.len() {
            if matrix[i][n] < target {
                continue;
            }

            if matrix[i][0] > target {
                return false;
            }

            let (mut start, mut end) = (0, n);
            while start <= end {
                let mid = (start + end) / 2;
                if matrix[i][mid] == target {
                    return true;
                } else if matrix[i][mid] > target {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            }

            return false;
        }

        false
    }
}
