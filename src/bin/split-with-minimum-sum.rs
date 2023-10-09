#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut num = num;
        let mut v = [0; 10];
        while num > 0 {
            v[(num % 10) as usize] += 1;
            num /= 10;
        }
        let (mut l, mut r) = (0, 0);
        let mut i = 1;
        let mut f = false;
        while i < v.len() {
            if v[i] > 0 {
                if f {
                    l = l * 10 + i as i32;
                } else {
                    r = r * 10 + i as i32;
                }
                v[i] -= 1;
                f = !f;
            } else {
                i += 1;
            }
        }

        l + r
    }
}
