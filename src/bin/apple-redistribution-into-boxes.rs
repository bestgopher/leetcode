#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut total: i32 = apple.into_iter().sum();
        let mut capacity = capacity;
        capacity.sort();
        let mut result = 0;
        while let Some(x) = capacity.pop() {
            total -= x;
            result += 1;
            if total <= 0 {
                break;
            }
        }

        result
    }
}
