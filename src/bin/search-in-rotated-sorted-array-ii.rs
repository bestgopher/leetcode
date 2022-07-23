#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    assert_eq!(
        true,
        Solution::search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 13, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            13
        )
    );
    assert_eq!(
        true,
        Solution::search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
            2
        )
    );
    assert_eq!(true, Solution::search(vec![1, 0, 1, 1, 1], 0));
    assert_eq!(false, Solution::search(vec![1], 0));
    assert_eq!(true, Solution::search(vec![1], 1));
}

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start <= end {
            let mid = (start + end) / 2;
            if nums[mid] == target {
                return true;
            }

            if start == 0 && end == 0 {
                return false;
            }

            if nums[start] == nums[mid] && nums[mid] == nums[end] {
                start += 1;
                end -= 1;
            } else if nums[start] <= nums[mid] {
                if nums[start] <= target && nums[mid] > target {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[nums.len() - 1] {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }

        false
    }
}
