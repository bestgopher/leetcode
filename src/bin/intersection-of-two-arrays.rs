#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();

        let mut result = vec![];

        let (mut index1, mut index2) = (0, 0);
        while index1 < nums1.len() && index2 < nums2.len() {
            if nums1[index1] == nums2[index2] {
                if result.len() == 0 || nums1[index1] != *result.last().unwrap() {
                    result.push(nums1[index1]);
                }
                index1 += 1;
                index2 += 1;
            } else if nums1[index1] > nums2[index2] {
                index2 += 1;
            } else {
                index1 += 1;
            }
        }
        result
    }
}
