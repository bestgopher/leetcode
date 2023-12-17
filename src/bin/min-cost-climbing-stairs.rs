#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut s1, mut s2) = (cost[0], cost[1]);

        for &i in cost[2..].iter() {
            let s = i + s1.min(s2);
            s1 = s2;
            s2 = s;
        }

        s1.min(s2)
    }
}
