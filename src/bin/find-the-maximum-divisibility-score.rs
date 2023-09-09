#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut r = i32::MAX;
        let mut max = 0;
        for i in divisors {
            let mut n = 0;
            for &j in nums.iter() {
                if j % i == 0 {
                    n += 1;
                }
            }

            if n > max {
                r = i as i32;
                max = n;
            } else if n == max {
                r = r.min(i as i32);
            }
        }

        r
    }
}
