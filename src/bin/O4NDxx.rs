#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    let n = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);

    assert_eq!(n.sum_region(2, 1, 4, 3), 8);
}

struct Solution;

struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    ///[
    ///     [3,0,1,4,2],
    ///     [5,6,3,2,1],
    ///     [1,2,0,1,5],
    ///     [4,1,0,1,7],
    ///     [1,0,3,0,5]
    /// ]
    ///
    /// [
    ///     [3,3,4,8,10],
    ///     [5,11,14,16,17],
    ///     [1,3,3,4,9],
    ///     [4,5,5,6,13],
    ///     [1,1,4,4,9]
    /// ]
    ///
    /// [
    ///     [0,0,0,0,0,0],
    ///     [0,3,3,4,8,10],
    ///     [0,8,14,18,24,27],
    ///     [0,9,17,21,28,36],
    ///     [0,13,22,26,34,49],
    ///     [0,14,23,30,38,58]
    /// ]
    ///
    /// sum = sum[row2 + 1][col2 + 1] - sum[row1][col2 + 1] - sum[row2 + 1][col1] + sum[row1][col1]
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut matrix = matrix;

        for i in 0..matrix.len() {
            for j in 1..matrix[0].len() {
                matrix[i][j] += matrix[i][j - 1];
            }
        }

        let mut m = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i != 0 {
                    matrix[i][j] += matrix[i - 1][j];
                }

                m[i + 1][j + 1] = matrix[i][j];
            }
        }

        Self { matrix: m }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.matrix[row2 as usize + 1][col2 as usize + 1]
            - self.matrix[row1 as usize][col2 as usize + 1]
            - self.matrix[row2 as usize + 1][col1 as usize]
            + self.matrix[row1 as usize][col1 as usize]
    }
}
