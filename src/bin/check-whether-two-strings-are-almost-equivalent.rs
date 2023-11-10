#![allow(dead_code, unused, unused_variables, non_snake_case)]

use tokio_stream::StreamExt;

fn main() {}

struct Solution;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut v = vec![0i32; 26];

        for &i in word1.as_bytes() {
            v[(i - b'a') as usize] += 1;
        }

        for &i in word2.as_bytes() {
            v[(i - b'a') as usize] -= 1;
        }

        v.into_iter().all(|x| x.abs() <= 3)
    }
}
