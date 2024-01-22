#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut s: Vec<u8> = num.to_string().as_bytes().into_iter().map(|x| *x).collect();
        let n = s.len();
        let mut max_idx = n - 1;
        let mut p = n;
        let mut q = 0;
        for i in (0..n - 1).rev() {
            if s[i] > s[max_idx] {
                // s[i] 是目前最大数字
                max_idx = i;
            } else if s[i] < s[max_idx] {
                // s[i] 右边有比它大的
                p = i;
                q = max_idx; // 更新 p 和 q
            }
        }
        if p == n {
            // 这意味着 s 是降序的
            return num;
        }
        s.swap(p, q); // 交换 s[p] 和 s[q]
        unsafe { String::from_utf8_unchecked(s) }.parse().unwrap()
    }
}
