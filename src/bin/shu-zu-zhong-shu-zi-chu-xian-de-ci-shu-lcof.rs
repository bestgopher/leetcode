#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/shu-zu-zhong-shu-zi-chu-xian-de-ci-shu-lcof/solution/jian-zhi-offer-56-i-shu-zu-zhong-shu-zi-tykom/
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut x = 0;

        for &i in nums.iter() {
            x ^= i;
        }

        let mut m = 1;
        while x & m != m {
            m <<= 1;
        }

        let mut v = vec![0; 2];

        for &i in nums.iter() {
            if i & m == m {
                v[0] ^= i;
            } else {
                v[1] ^= i;
            }
        }

        v
    }
}
