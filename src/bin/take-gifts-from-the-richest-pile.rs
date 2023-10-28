#![allow(dead_code, unused, unused_variables, non_snake_case)]

extern crate core;

use core::f64;

fn main() {}

struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut iter = gifts.into_iter().fuse();
        let mut sum = iter.clone().map(|x| x as i64).sum();
        let mut set = iter.collect::<std::collections::BinaryHeap<_>>();

        for i in 0..k {
            let m = set.pop().unwrap();
            sum -= m as i64 - (f64::sqrt(m as f64) as i64);
            set.push(f64::sqrt(m as f64) as i32);
        }

        sum
    }
}
