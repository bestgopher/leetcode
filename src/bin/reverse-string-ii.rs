#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s;
        let mut start = 0;
        unsafe {
            let mut d = s.as_bytes_mut();
            while start < d.len() {
                let mut b = (start + k as usize - 1).min(d.len() - 1);
                let mut a = start;
                d[a..=b].reverse();

                start += 2 * k as usize;
            }
        }

        s
    }
}
