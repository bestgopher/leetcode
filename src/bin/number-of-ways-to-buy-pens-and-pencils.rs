#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let (max, min) = if cost1 > cost2 {
            (cost1 as i64, cost2 as i64)
        } else {
            (cost2 as i64, cost1 as i64)
        };

        let mut r = 0;
        let mut i = 0;

        while i * max <= total as i64 {
            r += (total as i64 - max * i) / min + 1;
            i += 1;
        }

        r
    }
}
