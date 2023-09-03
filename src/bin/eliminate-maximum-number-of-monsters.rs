#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut s: Vec<_> = dist
            .into_iter()
            .zip(speed)
            .map(|(x, y)| if x % y == 0 { x / y } else { x / y + 1 })
            .collect();

        s.sort();

        let mut r = 1;

        for i in 0..s.len() - 1 {
            if i as i32 + 1 >= s[i] && s[i] == s[i + 1] {
                break;
            }

            r += 1;
        }

        r
    }
}
