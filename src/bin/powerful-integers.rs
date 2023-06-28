#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut result = std::collections::HashSet::new();
        let mut i = 0;

        while x.pow(i) <= bound {
            let mut j = 0;
            while x.pow(i) + y.pow(j) <= bound {
                result.insert(x.pow(i) + y.pow(j));
                if y == 1 {
                    break;
                }
                j += 1;
            }
            i += 1;

            if x == 1 {
                break;
            }
        }

        result.into_iter().collect()
    }
}
