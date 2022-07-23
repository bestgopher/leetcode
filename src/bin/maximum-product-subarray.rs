#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(4, Solution::max_product(vec![3, -1, 4]));
    assert_eq!(0, Solution::max_product(vec![-2, 0]));
    assert_eq!(-2, Solution::max_product(vec![-2]));
    assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
    assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
}

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut v = vec![0; nums.len()];
        v[0] = nums[0];

        for i in 1..nums.len() {
            if nums[i] != 0 {
                v[i] = if v[i - 1] == 0 {
                    nums[i]
                } else {
                    nums[i] * v[i - 1]
                };
            }
        }

        let mut result = v[0];
        let mut negative = 0;

        for i in v.into_iter() {
            if i < 0 {
                result = result.max(i / if negative == 0 { 1 } else { negative });
                negative = if negative == 0 { i } else { negative.max(i) };
            } else {
                result = result.max(i);
                if i == 0 {
                    negative = 0;
                }
            }
        }

        result
    }
}
