#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".into();
        }
        let mut s = String::new();
        let mut n = n;
        while n != 0 {
            let mut x = n % (-2);
            n /= -2;
            if x == -1 {
                n += 1;
                x = 1;
            }

            s.push_str(&x.to_string());
        }

        unsafe {
            s.as_bytes_mut().reverse();
        }
        s
    }
}
