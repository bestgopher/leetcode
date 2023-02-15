#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!(
        "{}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
    println!("{}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    println!("{}", Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));

    println!(
        "{}",
        Solution::length_of_lis1(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
    println!("{}", Solution::length_of_lis1(vec![0, 1, 0, 3, 2, 3]));
    println!("{}", Solution::length_of_lis1(vec![7, 7, 7, 7, 7, 7, 7]));
}

struct Solution;

impl Solution {
    /// 动态规划
    /// dp[i] = max(dp[j] + 1, dp[i]), 0 <= j <= i && nums[j] < nums[i]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut v = vec![1; nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    v[i] = v[i].max(v[j] + 1);
                }
            }
        }

        v.into_iter().fold(0, |x, y| x.max(y))
    }

    /// 动态规划 + 二分查找
    /// 比如序列是78912345，前三个遍历完以后tail是789，
    /// 这时候遍历到1，就得把1放到合适的位置，
    /// 于是在tail二分查找1的位置，变成了189（如果序列在此时结束，因为res不变，所以依旧输出3），
    /// 再遍历到2成为129，然后是123直到12345
    ///
    /// 很具小巧思。新建数组 cell，用于保存最长上升子序列。
    /// 对原序列进行遍历，将每位元素二分插入 cell 中。
    /// 如果 cell 中元素都比它小，将它插到最后
    /// 否则，用它覆盖掉比它大的元素中最小的那个。
    /// 总之，思想就是让 cell 中存储比较小的元素。这样，cell 未必是真实的最长上升子序列，但长度是对的。
    pub fn length_of_lis1(nums: Vec<i32>) -> i32 {
        let mut v = vec![nums[0]];

        for &num in &nums[1..] {
            if num > v[v.len() - 1] {
                v.push(num);
                continue;
            }

            let (mut i, mut j) = (0, v.len() - 1);

            while i < j {
                let middle = (i + j) / 2;
                if v[middle] < num {
                    i = middle + 1;
                } else {
                    j = middle;
                }
            }

            v[i] = num;
        }

        v.len() as i32
    }
}
