#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (mut row, mut col) = (
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        );

        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    row.entry(i).and_modify(|x| *x += 1).or_insert(1);
                    col.entry(j).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    match (row.get(&i), col.get(&j)) {
                        (Some(&x), Some(&y)) if x > 1 || y > 1 => result += 1,
                        _ => {}
                    }
                }
            }
        }

        result
    }
}
