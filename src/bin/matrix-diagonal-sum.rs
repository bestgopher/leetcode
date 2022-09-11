#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let (mut i, mut j, mut sum) = (0, mat.len() - 1, 0);

        while i < j {
            sum += mat[i][i];
            sum += mat[i][j];
            sum += mat[j][i];
            sum += mat[j][j];
            i += 1;
            j -= 1;
        }
        sum += (mat[i][i] * (mat.len() as i32 & 1));
        sum
    }
}
