#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    /// 从后往前遍历，
    /// 保存最大的值，
    /// 用最大值-当前值就为利润
    /// 保存利润的最大值
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut max, mut profit) = (0, 0);
        for i in (0..prices.len()).rev() {
            if i == prices.len() - 1 {
                max = prices[i];
            } else {
                if prices[i] > max {
                    max = prices[i];
                } else {
                    if max - prices[i] > profit {
                        profit = max - prices[i];
                    }
                }
            }
        }

        profit
    }
}
