#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let x = nums.iter().fold(0, |x, y| x ^ y);
        let y = x & -x;
        let (mut type1, mut type2) = (0, 0);
        for i in nums {
            if i & y > 0 {
                type1 ^= i;
            } else {
                type2 ^= i;
            }
        }

        vec![type1, type2]
    }
}
