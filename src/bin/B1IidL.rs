#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, arr.len() - 1);

        while start < end {
            let mut middle = (start + end) / 2;
            if arr[middle] > arr[middle - 1] && arr[middle] > arr[middle + 1] {
                return middle as i32;
            } else if arr[middle] > arr[middle - 1] && arr[middle] < arr[middle + 1] {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        unreachable!()
    }
}
