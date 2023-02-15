#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(Solution::max_profit(vec![1]), 0);
}

struct Solution;

impl Solution {
    /// 第i天有几种状态
    /// 1.持有股票，这时可能是持有的第i-1天的股票，或者是新买的股票
    /// 2.没有股票且处于冻结期(当天把股票卖了)
    /// 3.没有股票且不处于冻结期
    ///
    /// f[i][0]代表第一种情况
    /// f[i][2]代表第二种情况
    /// f[i][3]代表第三种情况
    ///
    /// f[i][0] = max(f[i-1][2]-price, f[i-1][0])
    /// f[i][1] = f[i-1][0]+price
    /// f[i][2] = max(f[i-1][2], f[i-1][1])
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 使用三个变量替代
        // let mut v = vec![vec![0; 3]; prices.len()];
        // v[0][0] = -prices[0];
        let (mut v0, mut v1, mut v2) = (-prices[0], 0, 0);

        for index in 1..prices.len() {
            let new_v0 = v0.max(v2 - prices[index]);
            let new_v1 = v0 + prices[index];
            let new_v2 = v1.max(v2);

            v0 = new_v0;
            v1 = new_v1;
            v2 = new_v2;
        }
        v0.max(v1).max(v2)
    }
}
