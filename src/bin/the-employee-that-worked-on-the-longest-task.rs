#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut t = 0;
        let mut r = 0;

        for i in 0..logs.len() {
            if i == 0 {
                t = logs[i][0];
                r = logs[i][1];
            } else if logs[i][1] - logs[i - 1][1] > r {
                t = logs[i][0];
                r = logs[i][1] - logs[i - 1][1];
            } else if logs[i][1] - logs[i - 1][1] == r {
                t = t.min(logs[i][0]);
            }
        }

        t
    }
}
