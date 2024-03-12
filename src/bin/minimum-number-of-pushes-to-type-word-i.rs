#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::minimum_pushes("abcde".into()), 5);
    assert_eq!(Solution::minimum_pushes("xycdefghij".into()), 12);
    assert_eq!(
        Solution::minimum_pushes("amrvxnhsewkoipjyuclgtdbfq".into()),
        52
    );
}

struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut r = 0;
        let mut len = word.len();
        match len {
            0..=8 => len as i32,
            9..=16 => 8 + (len as i32 - 8) * 2,
            17..=24 => 24 + (len as i32 - 16) * 3,
            25..=26 => 48 + (len as i32 - 24) * 4,
            _ => unreachable!(),
        }
    }
}
