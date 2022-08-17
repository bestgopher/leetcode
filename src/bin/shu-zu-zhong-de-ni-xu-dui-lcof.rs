#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 暴力解法
    pub fn reverse_pairs1(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i] > nums[j] {
                    result += 1;
                }
            }
        }

        result
    }

    /// 使用merge sort排序
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut new = vec![0; nums.len()];
        Self::merge_sort(&nums, &mut new)
    }

    fn merge_sort(nums: &[i32], new: &mut [i32]) -> i32 {
        let mut num = 0;
        if nums.len() <= 1 {
            if nums.len() == 1 {
                new[0] = nums[0];
            }
            return num;
        }

        let middle = nums.len() / 2;

        let mut left_vec = vec![0; middle];
        let left = Self::merge_sort(&nums[..middle], &mut left_vec);

        let mut right_vec = vec![0; nums.len() - middle];
        let right = Self::merge_sort(&nums[middle..], &mut right_vec);

        let (mut i, mut j, mut k) = (0, 0, 0);

        loop {
            match (left_vec.get(i), right_vec.get(j)) {
                (Some(x), Some(y)) => {
                    if *x > *y {
                        j += 1;
                        num += (left_vec.len() - i) as i32;
                        new[k] = *y;
                    } else {
                        i += 1;
                        new[k] = *x;
                    }
                }

                (Some(x), None) => {
                    new[k] = *x;
                    i += 1;
                }

                (None, Some(y)) => {
                    new[k] = *y;
                    j += 1;
                }

                (None, None) => break,
            }

            k += 1;
        }
        num + left + right
    }
}
