#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let mut pattern = pattern.bytes();
        let a = pattern.next().unwrap();
        let b = pattern.next().unwrap();

        let mut count = [0; 2];

        let mut result = 0;

        for i in text.bytes() {
            if i == a {
                count[0] += 1;
            } else if i == b {
                result += count[0];
                count[1] += 1;
            }
        }
        if a != b {
            result + count[0].max(count[1])
        } else {
            count[0] * (count[0] + 1) / 2
        }
    }
}
