#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for i in operations {
            gem[i[1] as usize] += gem[i[0] as usize] / 2;
            gem[i[0] as usize] -= gem[i[0] as usize] / 2;
        }

        let (mut max, mut min) = (gem[0], gem[0]);

        for i in 1..gem.len() {
            max = max.max(gem[i]);
            min = min.min(gem[i]);
        }

        max - min
    }
}
