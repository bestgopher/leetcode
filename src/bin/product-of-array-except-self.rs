#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
}

struct Solution;

impl Solution {
    /// 常规解法
    pub fn product_except_self1(nums: Vec<i32>) -> Vec<i32> {
        let sum = nums.iter().fold(1, |x, y| x * *y);

        nums.iter().fold(vec![], |mut x, y| {
            x.push(sum / (*y));
            x
        })
    }

    /// 左右乘积，某个位置的积 == 左边*右边
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            if i == 0 {
                v.push(1);
            } else if i == 1 {
                v.push(nums[i - 1]);
            } else {
                v.push(nums[i - 1] * v[i - 1]);
            }
        }

        let mut x = 1;

        for i in (0..nums.len()).rev() {
            if i == nums.len() - 1 {
                x = nums[i];
                continue;
            } else {
                v[i] = v[i] * x;
                x *= nums[i];
            }
        }

        v
    }
}
