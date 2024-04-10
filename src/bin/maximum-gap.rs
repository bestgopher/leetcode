#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut heap: std::collections::BinaryHeap<i32> = nums.into_iter().collect();
        let mut r = 0;
        let mut prev = heap.pop().unwrap();
        while let Some(x) = heap.pop() {
            r = r.max(prev - x);
            prev = x;
        }

        r
    }
}
