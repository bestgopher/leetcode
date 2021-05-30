fn main() {}

struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut s = 0;
        let mut n = n;
        while n > 0 {
            s += n / 5;
            n /= 5;
        }
        s
    }
}
