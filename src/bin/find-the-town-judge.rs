#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::future::ready;

fn main() {}

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut m = std::collections::HashSet::new();

        for v in trust {
            map.entry(v[1]).and_modify(|x| *x += 1).or_insert(1);
            m.insert(v[0]);
        }

        let mut result = None;
        for i in 1..=n {
            let v = map.get(&i).unwrap_or(&0);
            if *v == n - 1 && !m.contains(&i) {
                if result.is_none() {
                    result = Some(i);
                } else {
                    return -1;
                }
            }
        }

        result.unwrap_or(-1)
    }
}
