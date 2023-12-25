#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let x = (tomato_slices - 2 * cheese_slices) / 2;
        let y = (4 * cheese_slices - tomato_slices) / 2;

        if x < 0 || y < 0 || 4 * x + 2 * y != tomato_slices || x + y != cheese_slices {
            vec![]
        } else {
            vec![x, y]
        }
    }
}
