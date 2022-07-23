#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, numbers.len() - 1);

        while start < end {
            let middile = start + (end - start) / 2;
            if numbers[middile] < numbers[end] {
                end = middile;
            } else if numbers[middile] > numbers[end] {
                start = middile + 1;
            } else {
                end -= 1;
            }
        }

        numbers[start]
    }
}
