#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cmp::max;

fn main() {}

struct Solution;

impl Solution {
    pub fn max_alive_year(birth: Vec<i32>, death: Vec<i32>) -> i32 {
        let mut s = std::collections::HashMap::new();
        let mut max_year = i32::MAX;
        let mut max_num = 0;
        for i in 0..birth.len() {
            for j in birth[i]..=death[i] {
                s.entry(j).and_modify(|x| *x += 1).or_insert(1);
                if s[&j] > max_num {
                    max_num = s[&j];
                    max_year = j;
                }

                match s[&j].cmp(&max_num) {
                    std::cmp::Ordering::Equal => max_year = max_year.min(j),
                    std::cmp::Ordering::Greater => {
                        max_year = j;
                        max_num = s[&j];
                    }
                    _ => {}
                }
            }
        }

        max_year
    }
}
