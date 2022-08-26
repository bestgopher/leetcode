#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let (mut i, mut j) = (matrix.len() - 1, 0);

        loop {
            match target.cmp(&matrix[i][j]) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if j == matrix[0].len() - 1 {
                        return false;
                    }

                    j += 1;
                }
                std::cmp::Ordering::Less => {
                    if i == 0 {
                        return false;
                    }

                    i -= 1;
                }
            }
        }

        false
    }
}
