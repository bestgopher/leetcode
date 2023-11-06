#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        for i in nums {
            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut r = 0;

        for (_, v) in hash {
            if v == 1 {
                return -1;
            }

            if v % 3 == 0 {
                r += v / 3;
            } else {
                r += v / 3 + 1;
            }
        }

        r
    }
}
