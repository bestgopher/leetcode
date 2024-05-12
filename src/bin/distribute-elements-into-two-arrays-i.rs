#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums1 = Vec::with_capacity(nums.len());
        let mut nums2 = vec![];

        nums1.push(nums[0]);
        nums2.push(nums[1]);

        for i in 2..nums.len() {
            if nums1[nums1.len() - 1] > nums2[nums2.len() - 1] {
                nums1.push(nums[i]);
            } else {
                nums2.push(nums[i]);
            }
        }

        nums1.extend(nums2);

        nums1
    }
}
