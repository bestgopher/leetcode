#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 倒推法
    /// n等于1时，一定返回下标为0的
    /// 最后一个元素的下标一定会一直往前移动m位，即f(n-1)的下标为f(n) - m，因此f(n) = f(n-1) + m
    /// 又因为会溢出，所以需要对n取模，所以f(n) = (f(n-1) + m) % n
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut v = 0;

        for i in 2..n + 1 {
            v = (v + m) % i;
        }

        v
    }

    pub fn last_remaining_1(n: i32, m: i32) -> i32 {
        let mut v = (0..n).collect::<Vec<i32>>();

        let mut index = 0; // 开始的索引，开始从0计数
        while v.len() > 1 {
            index = (index + m as usize - 1) % v.len();

            v.remove(index);
        }

        v[0]
    }
}
