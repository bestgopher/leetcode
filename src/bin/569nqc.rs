#![allow(dead_code, unused, unused_variables, non_snake_case)]

use core::time;

fn main() {}

struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        // 因为只有1440种可能，因此大于这个值的话必定有2个相同的，直接返回0即可
        // 鸽巢原理/抽屉原理
        if time_points.len() > 1440 {
            return 0;
        }

        let mut time_points = time_points;
        time_points.sort();

        let (h1, m1) = time_points[time_points.len() - 1].split_once(':').unwrap();
        let (h2, m2) = time_points[0].split_once(':').unwrap();

        let mut t = h2.trim_start_matches('0').parse::<i32>().unwrap_or(0) * 60
            + m2.parse::<i32>().unwrap_or(0)
            + (23 - h1.trim_start_matches('0').parse::<i32>().unwrap_or(0)) * 60
            + (60 - m1.trim_start_matches('0').parse::<i32>().unwrap_or(0));

        for i in 1..time_points.len() {
            let (h1, m1) = time_points[i - 1].split_once(':').unwrap();
            let (h2, m2) = time_points[i].split_once(':').unwrap();

            t = t.min(
                (h2.trim_start_matches('0').parse::<i32>().unwrap_or(0)
                    - h1.trim_start_matches('0').parse::<i32>().unwrap_or(0))
                    * 60
                    + m2.trim_start_matches('0').parse::<i32>().unwrap_or(0)
                    - m1.trim_start_matches('0').parse::<i32>().unwrap_or(0),
            );
        }

        t
    }
}
