fn main() {}

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;

        while n > 1 || n < -1 {
            if n % 3 == 0 {
                n /= 3;
            } else {
                return false;
            }
        }

        n == 1
    }
}
