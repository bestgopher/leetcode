#![allow(dead_code, unused, unused_variables)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();

        let mut diff = arr[1] - arr[0];
        let mut v = vec![vec![arr[0], arr[1]]];
        for i in 2..arr.len() {
            match (arr[i] - arr[i - 1]).cmp(&diff) {
                std::cmp::Ordering::Less => {
                    diff = arr[i] - arr[i - 1];
                    v = vec![];
                    v.push(vec![arr[i - 1], arr[i]]);
                }

                std::cmp::Ordering::Equal => v.push(vec![arr[i - 1], arr[i]]),
                std::cmp::Ordering::Greater => {}
            }
        }
        v
    }
}
