#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut s, mut n) = (0, 0); // 奇数、偶数的个数

        for i in position {
            if i % 2 == 0 {
                s += 1;
            } else {
                n += 1;
            }
        }

        s.min(n)
    }
}
