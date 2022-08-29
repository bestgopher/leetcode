#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/1nzheng-shu-zhong-1chu-xian-de-ci-shu-lcof/solution/mian-shi-ti-43-1n-zheng-shu-zhong-1-chu-xian-de-2/
    /// 找规律
    pub fn count_digit_one(n: i32) -> i32 {
        let (mut n, mut r) = (n, 0);
        let mut digit = 1; // 位数
        let mut low = 0;
        while n > 0 {
            let mut cur = n % 10;
            let mut high = n / 10;
            match cur {
                0 => r += (high * digit),
                1 => r += (high * digit + low + 1),
                2..=9 => r += ((high + 1) * digit),
                _ => unreachable!(),
            }
            low += (cur * digit);
            digit *= 10;
            n = (n - cur) / 10;
        }

        r
    }
}
