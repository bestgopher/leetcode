#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(10, Solution::get_sum(4, 6));
    assert_eq!(10, Solution::get_sum(12, -2));
    assert_eq!(10, Solution::get_sum(0, 10));
    assert_eq!(10, Solution::get_sum(10, 0));
}

struct Solution;

impl Solution {
    // 反码：正数本身，负数非符号取反
    // 补码：正数本身，负数反码+1
    pub fn get_sum1(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }

        let (sum, carry) = (a ^ b, (a & b) << 1);
        Self::get_sum(sum, carry)
    }

    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut sum = a;
        let mut carry = b;
        while carry != 0 {
            let sum1 = sum ^ carry;
            carry = (sum & carry) << 1;
            sum = sum1;
        }
        sum
    }
}
