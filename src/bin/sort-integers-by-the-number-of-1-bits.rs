#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        arr.sort_by(|x, y| match x.count_ones().cmp(&y.count_ones()) {
            std::cmp::Ordering::Equal => x.cmp(y),
            i => i,
        });

        arr
    }
}
