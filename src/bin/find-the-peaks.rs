#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 1..mountain.len() - 1 {
            if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
                result.push(i as i32);
            }
        }

        result
    }
}
