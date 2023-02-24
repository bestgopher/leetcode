#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = [0; 26];

        for &v in tasks.iter() {
            count[(v as u8 - b'A') as usize] += 1
        }

        let max = count.iter().map(|x| *x).max().unwrap();

        let mut t = 0;
        let t = count.iter().filter(|x| **x == max).count(); // 与max数量一致的任务数量

        (tasks.len() as i32).max((n + 1) * (max - 1) + t as i32)
    }
}
