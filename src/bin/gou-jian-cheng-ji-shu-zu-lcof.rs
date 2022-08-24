#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn construct_arr1(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() {
            return vec![];
        }

        let mut v1 = vec![0; a.len()];
        let mut v2 = vec![0; a.len()];

        for i in 0..a.len() {
            if i == 0 {
                v1[i] = a[i];
                v2[a.len() - 1] = a[a.len() - 1];
            } else {
                v1[i] = a[i] * v1[i - 1];
                v2[a.len() - 1 - i] = a[a.len() - 1 - i] * v2[a.len() - i];
            }
        }

        let mut r = Vec::with_capacity(a.len());

        for i in 0..a.len() {
            if i == 0 {
                r.push(v2[i + 1]);
            } else if i == a.len() - 1 {
                r.push(v1[a.len() - 2]);
            } else {
                r.push(v1[i - 1] * v2[1 + i]);
            }
        }

        r
    }

    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        let mut r = vec![1; a.len()];

        for i in 1..a.len() {
            r[i] = a[i - 1] * r[i - 1];
        }
        let mut temp = 1;
        for i in (0..a.len()).rev() {
            r[i] *= temp;
            temp *= a[i];
        }

        r
    }
}
