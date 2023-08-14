#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sort_string(mut s: String) -> String {
        let mut count = [0; 26];
        for &i in s.as_bytes() {
            count[(i - b'a') as usize] += 1;
        }
        let mut slice = unsafe { s.as_bytes_mut() };
        let mut start = 0;
        let mut flag = false; // true为选最大，false为选最小
        while start < slice.len() {
            if !flag {
                for i in 0..26 {
                    if count[i] > 0 {
                        slice[start] = i as u8 + b'a';
                        count[i] -= 1;
                        start += 1;
                    }
                }
            } else {
                for i in (0..26).rev() {
                    if count[i] > 0 {
                        slice[start] = i as u8 + b'a';
                        count[i] -= 1;
                        start += 1;
                    }
                }
            }

            flag = !flag;
        }

        s
    }
}
