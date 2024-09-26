#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let (mut a1, mut a2) = (0, 0);

        for mut i in nums {
            a1 += i;

            while i > 0 {
                a2 += i % 10;
                i /= 10;
            }
        }

        (a1 - a2).abs()
    }
}
