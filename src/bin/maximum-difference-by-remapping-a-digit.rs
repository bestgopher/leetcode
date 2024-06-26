#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        Self::max(num) - Self::min(num)
    }

    fn max(mut num: i32) -> i32 {
        let mut s = vec![];
        let mut first = 0;
        while num > 0 {
            let x = num % 10;
            if x != 9 {
                first = x;
            }
            s.push(x);
            num /= 10;
        }

        let mut n = 0;
        while let Some(x) = s.pop() {
            if x == first {
                n = n * 10 + 9;
            } else {
                n = n * 10 + x;
            }
        }
        n
    }

    fn min(mut num: i32) -> i32 {
        let mut s = vec![];
        while num > 0 {
            s.push(num % 10);
            num /= 10;
        }

        let mut n = 0;
        let mut first = s[s.len() - 1];
        while let Some(x) = s.pop() {
            if x == first {
                n = n * 10 + 0;
            } else {
                n = n * 10 + x;
            }
        }
        n
    }
}
