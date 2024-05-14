#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut count = std::collections::HashMap::new();
        for i in tasks {
            *count.entry(i).or_insert(0) += 1;
        }

        let mut result = 0;
        for &i in count.values() {
            if i < 2 {
                return -1;
            }

            match i % 3 {
                0 => result += i / 3,
                1 => result += (i - 4) / 3 + 2,
                2 => result += i / 3 + 1,
                _ => unreachable!(),
            }
        }

        result
    }
}
