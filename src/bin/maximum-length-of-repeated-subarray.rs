#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut v = vec![vec![0; nums1.len()]; nums2.len()];
        let mut res = 0;
        for (i, &v1) in nums1.iter().enumerate() {
            for (j, &v2) in nums2.iter().enumerate() {
                if v1 == v2 {
                    v[j][i] = if j != 0 && i != 0 {
                        v[j - 1][i - 1] + 1
                    } else {
                        1
                    };
                }
                res = res.max(v[j][i])
            }
        }

        res
    }
}
