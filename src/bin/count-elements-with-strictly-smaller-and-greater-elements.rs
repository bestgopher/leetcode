#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_elements1(nums: Vec<i32>) -> i32 {
        let (mut min, mut max) = (i32::MAX, i32::MIN);

        for &i in nums.iter() {
            min = min.min(i);
            max = max.max(i);
        }

        let mut result = 0;

        for i in nums {
            if i > min && i < max {
                result += 1;
            }
        }

        result
    }

    /// 统计最大值和最小值的个数，然后总数-最大值和最小值的个数即为结果
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let (mut min, mut max) = (nums[0], nums[0]);
        let (mut min_count, mut max_count) = (1, 1);

        for &i in nums[1..].iter() {
            if i < min {
                min = i;
                min_count = 1;
            } else if i == min {
                min_count += 1;
            }

            if i > max {
                max = i;
                max_count = 1;
            } else if i == max {
                max_count += 1;
            }
        }

        if min == max {
            0
        } else {
            nums.len() as i32 - max_count - min_count
        }
    }
}
