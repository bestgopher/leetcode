#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut d: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();
        d.sort();
        let mut worker = worker;
        worker.sort();
        let mut max = 0;
        let mut result = 0;
        let mut j = 0;
        for i in worker {
            while j < d.len() && d[j].0 <= i {
                max = max.max(d[j].1);
                j += 1;
            }

            result += max;
        }

        result
    }
}
