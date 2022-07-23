#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::is_power_of_two(16));
    println!("{}", Solution::is_power_of_two(15));
    println!("{}", Solution::is_power_of_two(14));
    println!("{}", Solution::is_power_of_two(5));
}

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n - 1) & n == 0
    }
}
