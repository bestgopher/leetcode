#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let (mut i, mut j) = (0, 0);

        while i < nums1.len() && j < nums2.len() {
            if nums1[i][0] < nums2[j][0] {
                result.push(nums1[i].clone());
                i += 1;
            } else if nums1[i][0] > nums2[j][0] {
                result.push(nums2[j].clone());
                j += 1;
            } else {
                result.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            }
        }

        if i < nums1.len() {
            result.extend_from_slice(&nums1[i..]);
        }

        if j < nums2.len() {
            result.extend_from_slice(&nums2[j..]);
        }

        result
    }
}
