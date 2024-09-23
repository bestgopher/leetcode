#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut result = values[0] + 0 + values[1] - 1;
        let mut i = (values[0] + 0).max(values[1] + 1);
        for index in 2..values.len() {
            result = result.max(values[index] - index as i32 + i);
            i = i.max(values[index] + index as i32);
        }

        result
    }
}
