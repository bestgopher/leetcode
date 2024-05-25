#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_indices1(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + index_difference as usize..nums.len() {
                if (nums[i] - nums[j]).abs() >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![-1, -1]
    }

    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let (mut max_id, mut min_id) = (0, 0);

        for j in index_difference as usize..nums.len() {
            let i = j - index_difference as usize;
            if nums[i] > nums[max_id] {
                max_id = i;
            } else if nums[i] < nums[min_id] {
                min_id = i;
            }

            if nums[j] - nums[min_id] >= value_difference {
                return vec![min_id as i32, j as i32];
            }

            if nums[max_id] - nums[j] >= value_difference {
                return vec![max_id as i32, j as i32];
            }
        }

        vec![-1, -1]
    }
}
