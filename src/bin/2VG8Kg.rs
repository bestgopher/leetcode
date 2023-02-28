#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
        0
    );
}

struct Solution;

impl Solution {
    /// 两个指针 start和end，start和end的和为sum
    /// 如果sum >= target，则说明区间满足条件，start向前移动，sum减去start后移前下标位置的值
    /// 否则的话，end后移，sum加上当前end后移后当前下标的值
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, 1);
        let mut sum = nums[0];
        let mut ans = nums.len() + 1;

        loop {
            if sum >= target {
                ans = ans.min(end - start);
                sum -= nums[start];
                start += 1;
            } else {
                end += 1;
                if end > nums.len() {
                    break;
                }

                sum += nums[end - 1];
            }

            if ans == 1 {
                break;
            }
        }

        if ans == nums.len() + 1 {
            0
        } else {
            ans as i32
        }
    }
}
