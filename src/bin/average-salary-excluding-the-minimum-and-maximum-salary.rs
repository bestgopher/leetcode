#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let (mut sum, mut max, mut min) = (0, i32::MIN, i32::MAX);
        for &i in salary.iter() {
            sum += i;
            max = max.max(i);
            min = min.min(i);
        }

        (sum - max - min) as f64 / (salary.len() as f64 - 2f64)
    }
}
