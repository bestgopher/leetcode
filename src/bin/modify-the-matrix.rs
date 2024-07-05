#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;

        for i in 0..matrix[0].len() {
            let max = (0..matrix.len()).map(|x| matrix[x][i]).max().unwrap();

            for j in 0..matrix.len() {
                if matrix[j][i] == -1 {
                    matrix[j][i] = max;
                }
            }
        }

        matrix
    }
}
