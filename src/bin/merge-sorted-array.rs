#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m - 1, n - 1);
        for i in (0..nums1.len()).rev() {
            if m < 0 {
                nums1[i] = nums2[n as usize];
                n -= 1;
            } else if n < 0 {
                nums1[i] = nums1[m as usize];
                m -= 1;
            } else {
                if nums1[m as usize] > nums2[n as usize] {
                    nums1[i] = nums1[m as usize];
                    m -= 1;
                } else {
                    nums1[i as usize] = nums2[n as usize];
                    n -= 1;
                }
            }
        }
    }
}
