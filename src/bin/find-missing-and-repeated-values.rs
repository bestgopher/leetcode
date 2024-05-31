#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut hash = std::collections::HashMap::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                *hash.entry(grid[i][j]).or_insert(0) += 1;
            }
        }

        let mut result = vec![0, 0];
        for i in 1..=(grid.len() * grid.len()) as i32 {
            match hash.get(&i) {
                Some(&x) if x == 2 => result[0] = i,
                None => result[1] = i,
                _ => {}
            }
        }

        result
    }
}
