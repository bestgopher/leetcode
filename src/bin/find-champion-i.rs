#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut hash_set = std::collections::HashSet::new();
        let mut champion = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i == j {
                    continue;
                }

                let m = if grid[i][j] == 1 { i } else { j };
                if !hash_set.contains(&m) {
                    champion = m as i32;
                }

                hash_set.insert(if grid[i][j] == 1 { j } else { i });
            }
        }

        champion
    }
}
