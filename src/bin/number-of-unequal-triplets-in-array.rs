#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut h = std::collections::HashMap::new();
        let n = nums.len() as i32;
        for i in nums {
            h.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut ans = 0;
        let mut x = 0;
        // 当前元素的个数为v
        // 当前元素前面的元素个数为x
        // 当前元素后面的个数为n-x-v
        // 则当前元素为中间元素可以组成 x * v * (n - v - x) 个数
        for (_, v) in h.into_iter() {
            ans += x * v * (n - v - x);
            x += v;
        }

        ans
    }
}
