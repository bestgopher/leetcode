fn main() {}

struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut v = Vec::with_capacity(n as usize);
        for i in 1..=n / 2 {
            v.push(i);
            v.push(-i);
        }

        if n % 2 == 1 {
            v.push(0);
        }

        v
    }
}
