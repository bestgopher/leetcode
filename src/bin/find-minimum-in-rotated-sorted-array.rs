fn main() {
    assert_eq!(1, Solution::find_min(vec![3, 4, 5, 1, 2]));
    assert_eq!(0, Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    assert_eq!(11, Solution::find_min(vec![11, 13, 15, 17]));
}

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums[0] < *nums.last().unwrap() {
            return nums[0];
        }

        let (mut start, mut end) = (0, nums.len() - 1);

        while start < end {
            let middle = (start + end) / 2;
            if nums[middle] > nums[end] {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        nums[end]
    }
}
