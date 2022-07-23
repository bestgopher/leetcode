#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{:?}", Solution::combine(4, 2));
}

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::get(1, n, k)
    }

    fn get(start: i32, end: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        for i in (start..=end).rev() {
            if k != 1 {
                for mut j in Self::get(start, i - 1, k - 1) {
                    j.push(i);
                    result.push(j);
                }
            } else {
                result.push(vec![i]);
            }
        }
        result
    }
}
