#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut current = b' ';
        let mut count = 0;
        let mut result = String::new();
        for i in 0..num.len() {
            if num.as_bytes()[i] == current {
                count += 1;
                if count == 3 {
                    if result < num[i - 2..=i].to_owned() {
                        result = num[i - 2..=i].to_owned();
                    }
                    current = b' ';
                    count = 0;
                }
            } else {
                current = num.as_bytes()[i];
                count = 1;
            }
        }

        result
    }
}
