#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans;
        beans.sort_unstable();
        let sum: i64 = beans.iter().map(|x| *x as i64).sum();
        let mut r = i64::MAX;
        for i in 0..beans.len() {
            r = r.min(sum - (beans.len() - i) as i64 * (beans[i] as i64))
        }
        r
    }
}
