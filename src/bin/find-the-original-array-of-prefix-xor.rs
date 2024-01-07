#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{:?}", Solution::find_array(vec![5, 2, 0, 3, 1]));
}

struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; pref.len()];
        arr[0] = pref[0];
        let mut x = arr[0];
        for i in 1..pref.len() {
            arr[i] = x ^ pref[i];
            x ^= arr[i];
        }

        arr
    }
}
