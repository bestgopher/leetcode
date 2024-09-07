#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut map = [0; 26];

        for (i, v) in s.bytes().enumerate() {
            map[(v - b'a') as usize] = i;
        }

        let mut result = 0;

        for (i, v) in t.bytes().enumerate() {
            result += (i as i32 - map[(v - b'a') as usize] as i32).abs();
        }

        result
    }
}
