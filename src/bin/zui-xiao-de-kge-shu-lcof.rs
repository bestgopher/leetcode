#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::{iter::FromIterator, vec};

fn main() {
    println!("{:?}", Solution::get_least_numbers(vec![3, 2, 1], 2));
    println!("{:?}", Solution::get_least_numbers(vec![0, 1, 2, 1], 1));
    println!("{:?}", Solution::get_least_numbers((0..100).collect(), 15));
    println!(
        "{:?}",
        Solution::get_least_numbers(vec![0, 0, 1, 2, 4, 2, 2, 3, 1, 4], 8)
    );
}

/// 大顶堆
struct Heap {
    data: Vec<i32>,
    len: usize,
}

struct Solution;

impl Solution {
    pub fn get_least_numbers(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        Self::quick_sort(&mut arr, k as usize);
        (&arr[0..k as usize]).to_vec()
    }

    pub fn quick_sort(arr: &mut [i32], k: usize) {
        if arr.len() <= 1 || k == 0 {
            return;
        }

        let (mut base, mut begin, mut end) = (arr[0], 0, arr.len() - 1);
        while begin < end {
            while begin < end {
                if arr[end] > base {
                    end -= 1;
                } else {
                    arr.swap(begin, end);
                    begin += 1;
                    break;
                }
            }

            while begin < end {
                if arr[begin] < base {
                    begin += 1;
                } else {
                    arr.swap(begin, end);
                    end -= 1;
                    break;
                }
            }
        }

        arr[begin] = base;
        // [1,2,3,4,5] k=1 begin=2
        if begin == k || begin + 1 == k {
            return;
        } else if begin >= 1 + k {
            Self::quick_sort(&mut arr[..begin], k);
        } else {
            Self::quick_sort(&mut arr[begin + 1..], k - begin - 1);
        }
    }
}
