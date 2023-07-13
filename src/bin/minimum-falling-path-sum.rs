#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();

        for i in (0..n - 1).rev() {
            for j in 0..n {
                if j == 0 {
                    matrix[i][j] = matrix[i][j] + matrix[i + 1][j].min(matrix[i + 1][j + 1]);
                } else if j == n - 1 {
                    matrix[i][j] = matrix[i][j] + matrix[i + 1][j - 1].min(matrix[i + 1][j]);
                } else {
                    matrix[i][j] = matrix[i][j]
                        + matrix[i + 1][j - 1]
                            .min(matrix[i + 1][j])
                            .min(matrix[i + 1][j + 1]);
                }
            }
        }

        let mut r = matrix[0][0];

        for i in 1..n {
            r = r.min(matrix[0][i]);
        }

        r
    }
}
