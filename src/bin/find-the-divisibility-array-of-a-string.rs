#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut result = vec![0; word.len()];
        let m = m as i64;
        let mut s = 0i64;
        for (i, j) in word.bytes().enumerate() {
            s = (s * 10 + (j - b'0') as i64) % m;
            if s == 0 {
                result[i] = 1;
            }
        }

        result
    }
}
