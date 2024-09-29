#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let n = tickets[k as usize];

        tickets
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if i <= k as usize {
                    v.min(n)
                } else {
                    if v < n {
                        v
                    } else {
                        n - 1
                    }
                }
            })
            .sum()
    }
}
