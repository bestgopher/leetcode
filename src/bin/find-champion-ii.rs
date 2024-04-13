#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        let mut s: HashSet<_> = edges.iter().map(|x| x[1]).collect();
        if s.len() != n as usize - 1 {
            return -1;
        }

        let mut result = None;
        for i in edges {
            if !s.contains(&i[0]) {
                if result.is_some() && Some(i[0]) != result {
                    return -1;
                } else {
                    result = Some(i[0]);
                }
            }
        }

        result.unwrap_or(0)
    }
}
