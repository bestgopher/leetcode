#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        if (arr.len() as i32) < m * k {
            return false;
        }

        'l: for i in 0..arr.len() - (m as usize) {
            for j in i..m as usize + i {
                for v in 0..k as usize {
                    match arr.get(j + v * m as usize) {
                        Some(&x) if x != arr[j] => continue 'l,
                        None => continue 'l,
                        _ => {}
                    }
                }
            }

            return true;
        }

        false
    }
}
