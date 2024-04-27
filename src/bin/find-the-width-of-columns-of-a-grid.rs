#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];

        for i in 0..grid[0].len() {
            let mut r = -1;
            for j in 0..grid.len() {
                r = r.max(grid[j][i].to_string().len() as i32);
            }
            result.push(r);
        }

        result
    }
}
