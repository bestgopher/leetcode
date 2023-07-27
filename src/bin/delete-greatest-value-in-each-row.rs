#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|x| x.sort_by_key(|k| -k));

        let mut r = 0;

        for i in 0..grid[0].len() {
            let mut max = 0;
            for j in 0..grid.len() {
                max = max.max(grid[j][i]);
            }

            r += max;
        }

        r
    }
}
