use std::ptr::hash;

fn main() {}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;
        }

        let (mut a, mut b) = (1, 2);

        for i in 3..=n {
            let a1 = a;
            a = b;
            b = a1 + b;
        }
        b
    }
}
