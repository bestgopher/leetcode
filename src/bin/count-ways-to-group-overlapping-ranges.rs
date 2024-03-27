#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut ranges = ranges;
        ranges.sort();
        let mut max = -1;
        let mut m = 1;

        for range in ranges {
            if range[0] > max {
                m = m * 2 % 1000000007;
            }

            max = max.max(range[1]);
        }

        m
    }
}
