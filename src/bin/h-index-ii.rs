#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 在区间 (left, right] 内询问
        let n = citations.len();
        let mut left = 0;
        let mut right = n;
        while left < right {
            // 区间不为空
            // 循环不变量：
            // left 的回答一定为「是」
            // right+1 的回答一定为「否」
            let mid = (left + right + 1) / 2; // 保证 mid 在二分区间内
                                              // 引用次数最多的 mid 篇论文，引用次数均 >= mid
            if citations[n - mid] >= mid as i32 {
                left = mid; // 询问范围缩小到 (mid, right]
            } else {
                right = mid - 1; // 询问范围缩小到 (left, mid-1]
            }
        }
        // 根据循环不变量，left 现在是最大的回答为「是」的数
        left as i32
    }
}
