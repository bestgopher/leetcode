#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn majority_element1(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        nums[nums.len() / 2]
    }

    /// https://leetcode.cn/problems/shu-zu-zhong-chu-xian-ci-shu-chao-guo-yi-ban-de-shu-zi-lcof/solution/mian-shi-ti-39-shu-zu-zhong-chu-xian-ci-shu-chao-3/
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let mut x = None;
        let mut score = 0;

        for i in nums {
            if x.is_none() {
                x = Some(i);
            }

            match x {
                Some(s) if s == i => score += 1,
                _ => score -= 1,
            }

            if score == 0 {
                x = None;
            }
        }

        x.unwrap()
    }
}
