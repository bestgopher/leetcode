#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 前缀和
    /// 对于prefix_sum. 如果nums[i] 与 nums[i-1]的奇偶性不同，则prefix_sum[i] = 0, 否则为1
    /// 因此如果 nums[i] ~ nums[j] 是特殊子数组，则prefix_sum[i] == prefix_sum[j]
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_sum = vec![0];

        for i in 1..nums.len() {
            if nums[i] % 2 != nums[i - 1] % 2 {
                prefix_sum.push(prefix_sum[i - 1]);
            } else {
                prefix_sum.push(prefix_sum[i - 1] + 1);
            }
        }

        let mut result = vec![];
        for i in queries {
            if prefix_sum[i[0] as usize] == prefix_sum[i[1] as usize] {
                result.push(true);
            } else {
                result.push(false);
            }
        }

        result
    }
}
