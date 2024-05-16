#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let s = milestones.iter().map(|&x| x as i64).sum();
        let m = *milestones.iter().max().unwrap() as i64;
        if m > s - m + 1 {
            (s - m) * 2 + 1
        } else {
            s
        }
    }
}
