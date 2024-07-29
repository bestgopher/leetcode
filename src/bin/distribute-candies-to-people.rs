#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        impl Solution {
            pub fn distribute_candies(candies: i32, n: i32) -> Vec<i32> {
                let m = (((8.0 * candies as f64 + 1.0).sqrt() - 1.0) / 2.0) as i32;
                let k = m / n;
                let extra = m % n;
                let mut ans = (0..n)
                    .map(|i| {
                        let k = if i < extra { k + 1 } else { k };
                        k * (k - 1) / 2 * n + k * (i + 1)
                    })
                    .collect::<Vec<_>>();
                ans[extra as usize] += candies - m * (m + 1) / 2;
                ans
            }
        }
    }
}
