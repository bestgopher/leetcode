#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut r = i32::MAX;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    r = r.min(nums1[i]);
                } else if nums1[i] < nums2[j] {
                    r = r.min(nums1[i] * 10 + nums2[j]);
                } else {
                    r = r.min(nums1[i] + nums2[j] * 10);
                }
            }
        }

        r
    }

    pub fn min_number1(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut x, mut y) = (0i32, 0i32);

        for i in nums1 {
            x |= 1 << i;
        }

        for i in nums2 {
            y |= 1 << i;
        }

        if x & y != 0 {
            (x & y).trailing_zeros() as i32
        } else {
            (x.trailing_ones() * 10 + y.trailing_ones())
                .min(y.trailing_ones() * 10 + x.trailing_ones()) as i32
        }
    }
}
