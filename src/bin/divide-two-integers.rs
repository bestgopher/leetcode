#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
}

struct Solution;

impl Solution {
    // 设 x 为被除数，y为除数, z 为结果
    // 如果  x > y + y, 则 z+=z，y += y
    // 如果 x < y + y, 则 x -= y, 返回步骤二继续计算剩余的数
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let ans = Self::d(dividend as i64, divisor as i64);
        if ans > i32::MAX as i64 {
            i32::MAX
        } else if ans < i32::MIN as i64 {
            i32::MIN
        } else {
            ans as i32
        }
    }

    pub fn d(dividend: i64, divisor: i64) -> i64 {
        // 符号位， true表示为正数
        let is_na = dividend.signum() == divisor.signum();

        let (mut dividend, divisor) = (dividend.abs(), divisor.abs());

        if dividend < divisor {
            return 0;
        }

        let mut start = divisor;
        let mut ans = 1;
        while dividend > start + start {
            ans += ans;
            start += start;
        }

        ans += Self::d(dividend - start, divisor);

        if !is_na {
            ans = 0 - ans;
        }

        ans
    }
}
