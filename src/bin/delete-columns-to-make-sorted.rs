#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut count = 0;

        for i in 0..strs[0].len() {
            for j in 1..strs.len() {
                let s = strs[j - 1].as_bytes();
                let s1 = strs[j].as_bytes();
                if s[i] > s1[i] {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}
