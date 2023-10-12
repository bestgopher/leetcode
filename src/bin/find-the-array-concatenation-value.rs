#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let (mut start, mut end) = (0, nums.len() - 1);
        let mut r = 0;
        while start <= end {
            if start == end {
                r += nums[start] as i64;
                break;
            } else {
                let x = nums[start] as i64;
                let mut y = nums[end] as i64;
                let mut m = 1;
                while m <= y {
                    m *= 10;
                }
                r += (x * m) + y;
            }

            start += 1;
            end -= 1;
        }

        r
    }
}
