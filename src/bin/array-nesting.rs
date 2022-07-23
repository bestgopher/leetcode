#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(4, Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
}

struct Solution;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut s = vec![0; nums.len()];

        for i in 0..nums.len() {
            let n = Self::f(&nums[..], &mut s[..], i, nums[i]);
            if n > count {
                count = n;
            }

            s[i as usize] = n;

            if count == nums.len() as i32 {
                break;
            }
        }

        count
    }

    fn f(nums: &[i32], s: &mut [i32], n: usize, n1: i32) -> i32 {
        if n == n1 as usize {
            return 1;
        }

        if s[n as usize] != 0 {
            return s[n as usize];
        }

        let m = Self::f(nums, s, n, nums[n1 as usize]) + 1;
        s[n1 as usize] = m;
        m
    }
}
