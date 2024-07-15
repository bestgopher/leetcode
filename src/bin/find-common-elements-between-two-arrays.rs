#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut s, mut m) = (
            nums1
                .iter()
                .map(|x| *x)
                .collect::<std::collections::HashSet<_>>(),
            nums2
                .iter()
                .map(|x| *x)
                .collect::<std::collections::HashSet<_>>(),
        );
        let mut r1 = 0;
        let mut r2 = 0;

        for i in nums1 {
            if m.contains(&i) {
                r1 += 1;
            }
        }

        for i in nums2 {
            if s.contains(&i) {
                r2 += 1;
            }
        }

        vec![r1, r2]
    }
}
