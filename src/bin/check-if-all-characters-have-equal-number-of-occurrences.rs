#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut v = [0; 26];
        for &i in s.as_bytes() {
            v[(i - b'a') as usize] += 1;
        }

        let mut i = 0;

        for x in v {
            if x == 0 {
                continue;
            }

            if i != 0 {
                if i != x {
                    return false;
                }
            } else {
                i = x;
            }
        }

        true
    }
}
