#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        coins.sort_unstable();
        let mut ans = 0;
        let mut s = 1;
        let mut i = 0;
        while s <= target {
            if i < coins.len() && coins[i] <= s {
                s += coins[i];
                i += 1;
            } else {
                s *= 2; // 必须添加 s
                ans += 1;
            }
        }
        ans
    }
}
