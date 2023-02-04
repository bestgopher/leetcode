#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        5
    );
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]),
        3
    );
}

struct Solution;

impl Solution {
    /// 暴力搜索， 双指针
    /// p1 = arr[0], p2 = arr[1], 如果 p1+p2在arr存在，则 p1, p2 = p2, p1+p2，结果+1；如果不存在，则推出循环
    pub fn force_len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut set = arr
            .iter()
            .map(|x| *x)
            .collect::<std::collections::HashSet<_>>();
        let mut ans = 0;

        for i in 0..arr.len() - 2 {
            let mut _ans = -1;
            for j in i + 1..arr.len() - 1 {
                let mut current1 = arr[i];
                let mut current2 = arr[j];

                while set.contains(&(current1 + current2)) {
                    _ans = (_ans + 1).max(3);
                    let _current1 = current2;
                    current2 = current1 + current2;
                    current1 = _current1;
                }

                ans = ans.max(_ans);
                _ans = -1;
            }
        }

        ans
    }

    /// dp
    /// dp[i][j]表示以arr[i]和arr[j]开始的最大值
    /// 则 dp[i][j] = 1 + dp[j][arr[i] +arr[j]] if arr[i] + arr[j] 存在的话
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut map = arr
            .iter()
            .enumerate()
            .map(|(x, y)| (*y, x))
            .collect::<std::collections::HashMap<_, _>>();
        let mut ans = 0;

        let mut dp = vec![vec![-1; arr.len()]; arr.len()];

        for i in (0..arr.len() - 2).rev() {
            for j in (i + 1..arr.len() - 1) {
                let mut inner = 0;
                let mut sum = arr[i] + arr[j];
                match map.get(&sum) {
                    Some(&x) => inner = (dp[j][x] + 1).max(3),
                    None => inner = 0,
                }

                dp[i][j] = inner;
                ans = ans.max(inner);
            }
        }

        ans
    }
}
