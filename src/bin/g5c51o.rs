#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;

        let mut hash = std::collections::HashMap::new();
        for i in nums {
            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        hash.into_iter()
            .map(|(x, y)| (Reverse(y), x))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .take(k as usize)
            .map(|x| x.1)
            .collect()
    }
}
