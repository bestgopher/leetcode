#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::collections::BinaryHeap;

fn main() {}

struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<i32> = nums.into_iter().collect();

        let mut r = 0;
        for i in 0..k {
            let x = heap.pop().unwrap();
            r += x as i64;
            heap.push({
                if (x / 3) * 3 == x {
                    x / 3
                } else {
                    x / 3 + 1
                }
            });
        }

        r
    }
}
