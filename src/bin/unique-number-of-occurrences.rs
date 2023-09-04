#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut s: std::collections::HashMap<i32, i32> = Default::default();

        for i in arr {
            s.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut s1 = std::collections::HashSet::new();

        for (_, v) in s.into_iter() {
            if s1.contains(&v) {
                return false;
            }

            s1.insert(v);
        }

        true
    }
}
