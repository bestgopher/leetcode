#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(
        vec![3, 4],
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
    );
}

struct Solution;

impl Solution {
    /// 方法1：遍历整个数组，找出相应的下标位置
    pub fn search_range1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v = vec![-1; 2];

        for (index, value) in nums.into_iter().enumerate() {
            if value != target {
                continue;
            }

            if v[0] == -1 {
                v[0] = index as i32;
            }

            v[1] = index as i32;
        }

        v
    }

    /// 二分查找法
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v = vec![-1; 2];
        if nums.is_empty() {
            return v;
        }

        let (mut start, mut end) = (0, nums.len() - 1);

        while start <= end {
            let middle = (start + end) >> 1;
            if nums[middle] == target && (middle == 0 || nums[middle - 1] < target) {
                v[0] = middle as i32;
                break;
            } else if nums[middle] == target && middle != 0 && nums[middle - 1] == target {
                end = middle - 1;
            } else if nums[middle] > target {
                if middle == 0 {
                    break;
                }
                end = middle - 1;
            } else {
                start = middle + 1;
            }
        }

        let (mut start, mut end) = (0, nums.len() - 1);
        while start <= end {
            let middle = (start + end) >> 1;
            if nums[middle] == target && (middle == nums.len() - 1 || nums[middle + 1] > target) {
                v[1] = middle as i32;
                break;
            } else if nums[middle] == target
                && middle != nums.len() - 1
                && nums[middle + 1] == target
            {
                start = middle + 1;
            } else if nums[middle] > target {
                if middle == 0 {
                    break;
                }
                end = middle - 1;
            } else {
                start = middle + 1;
            }
        }

        v
    }
}
