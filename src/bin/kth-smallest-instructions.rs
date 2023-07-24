#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

use serde::__private::de;

fn main() {}

struct Solution;

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, mut k: i32) -> String {
        let mut max: Vec<u8> = (0..destination[1])
            .map(|_| b'H')
            .chain((0..destination[0]).map(|_| b'V'))
            .collect();

        while k > 0 {}

        unsafe { String::from_utf8_unchecked(max) }
    }
}
