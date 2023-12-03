#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut h = std::collections::HashMap::new();
        for i in deck {
            h.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut max = 0;

        for (_, j) in h {
            max = Self::gcd(max, j);
        }

        max >= 2
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else {
            Self::gcd(b % a, a)
        }
    }
}
