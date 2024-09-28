#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if let Some(Some(&x)) = grid.get(i + 1).map(|c| c.get(j)) {
                    if x != grid[i][j] {
                        return false;
                    }
                }

                if let Some(Some(&x)) = grid.get(i).map(|c| c.get(j + 1)) {
                    if x == grid[i][j] {
                        return false;
                    }
                }
            }
        }

        true
    }
}
