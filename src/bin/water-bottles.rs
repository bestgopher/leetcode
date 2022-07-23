#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(19, Solution::num_water_bottles(15, 4));
}

struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut a = num_bottles; // 剩余的空瓶

        while a >= num_exchange {
            num_bottles += 1; // 瓶数加1
            a -= num_exchange; // 空瓶数减num_exchange
            a += 1; // 瓶数加1
        }
        num_bottles
    }
}
