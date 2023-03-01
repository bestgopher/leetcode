#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    assert_eq!(Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]), 6);
}

struct Solution;

impl Solution {
    /// count记录0的数量-1的数量
    /// 当counter为0时，则说明此时满足条件
    /// 当counter不为0时，找到相同counter时的子数组，则减去相同counter的子数组剩余的数组就是0和1相同数量的子数组
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut count = 0; // 0的数量-1的数量

        let mut ans = 0;
        let mut map = std::collections::HashMap::new();

        for i in 0..nums.len() {
            if nums[i] == 0 {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                ans = ans.max(i as i32 + 1);
            } else {
                match map.get(&count) {
                    Some(&x) => ans = ans.max(i as i32 - x),
                    None => {}
                }

                map.entry(count).or_insert(i as i32);
            }
        }

        ans
    }
}
