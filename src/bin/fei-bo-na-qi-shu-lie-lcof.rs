fn main() {}

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        let (mut last, mut result) = (1, 1);
        for i in 2..n {
            let result1 = result + last;
            last = result;
            result = result1 % 1000000007;
        }

        result
    }
}
