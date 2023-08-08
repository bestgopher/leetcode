#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        let (mut max, mut min) = (0, 0); // max表示正数前缀和，min表示负数前缀和
        for i in nums {
            r = r.max(max + i).max((min + i).abs()).max(i.abs());

            if i > 0 {
                max += i;
                min = 0.min(i + min);
            } else if i < 0 {
                min += i;
                max = 0.max(max + i);
            }
        }

        r
    }
}
