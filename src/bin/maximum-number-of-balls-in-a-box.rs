#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut max_num = 0;
        let mut hash = std::collections::HashMap::new();
        for mut i in low_limit..=high_limit {
            let mut sum = 0;
            while i > 0 {
                sum += i % 10;
                i /= 10;
            }

            let x = hash.entry(sum).and_modify(|x| *x += 1).or_insert(1);
            if *x > max_num {
                max_num = *x;
            }
        }

        max_num
    }
}
