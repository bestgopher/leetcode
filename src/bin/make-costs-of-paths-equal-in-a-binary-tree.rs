#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]), 6);
    assert_eq!(Solution::min_increments(3, vec![5, 3, 3]), 0);
    assert_eq!(
        Solution::min_increments(
            15,
            vec![
                764, 1460, 2664, 764, 2725, 4556, 5305, 8829, 5064, 5929, 7660, 6321, 4830, 7055,
                3761,
            ],
        ),
        15735
    );
}

struct Solution;

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        let mut ans = 0;
        for i in (1..=n as usize / 2).rev() {
            // 从最后一个非叶节点开始算
            ans += (cost[i * 2 - 1] - cost[i * 2]).abs(); // 两个子节点变成一样的
            cost[i - 1] += cost[i * 2 - 1].max(cost[i * 2]); // 累加路径和
        }
        ans
    }
}
