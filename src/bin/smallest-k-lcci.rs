#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort();

        arr[..k as usize].into()
    }
}
