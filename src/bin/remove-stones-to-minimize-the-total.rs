#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(piles);

        for _ in 0..k {
            let x = heap.pop().unwrap();
            heap.push(if x % 2 == 0 { x / 2 } else { x / 2 + 1 });
        }

        heap.into_iter().sum()
    }
}
