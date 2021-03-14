use std::os::macos::raw::stat;

fn main() {
    assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8));
    assert_eq!(-1, Solution::search(vec![1, 3], 0));
    assert_eq!(-1, Solution::search(vec![1], 0));
    assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    assert_eq!(0, Solution::search(vec![5, 1, 3], 5));
    println!("xxxxx");
    assert_eq!(2, Solution::search(vec![5, 1, 3], 3));
}

struct Solution;

impl Solution {
    /// 先判断终点落在上半区还行下半区
    /// 在判断target在上半区还是下半区
    /// 要先判断中点落在哪个分区里面
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);
        let mut middle = (start + end) >> 1;

        while start < end {
            if nums[middle] == target {
                break;
            } else if nums[middle] > target {
                // 如果middle在前半区且target大于等于第一个元素，或者middle在后半区
                if (nums[start] < nums[middle] && target >= nums[start]) || nums[start] > nums[middle] {
                    end = middle;
                } else {
                    start = middle + 1;
                }
            } else {
                if (nums[end] > nums[middle] && nums[end] >= target) || nums[end] < nums[middle] {
                    start = middle + 1;
                } else {
                    end = middle;
                }
            }
            middle = (start + end) >> 1;
        }

        if nums[middle] == target { middle as i32 } else { -1 }
    }
}
