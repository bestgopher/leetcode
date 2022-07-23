#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut count = 0;

        for i in nums {
            if i == 1 {
                count += 1;
            } else {
                r = r.max(count);
                count = 0
            }
        }
        r
    }
}
