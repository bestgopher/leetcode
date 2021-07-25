fn main() {}

struct Solution;

impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        let mut n = n;
        // 利用布尔短路的思想，n == 0 时，不会允许&&后面的表达式
        if n > 0 {
            n += Self::sum_nums(n - 1);
        }

        n
    }
}
