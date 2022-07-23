#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::is_happy(19));
    println!("{}", Solution::is_happy(2));
}

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;

        let mut h = std::collections::HashSet::new();
        loop {
            let mut r = 0;
            while n > 0 {
                r += (n % 10).pow(2);
                n /= 10;
            }
            if r == 1 {
                return true;
            }

            if h.contains(&r) {
                return false;
            }

            h.insert(r);
            n = r;
        }
    }
}
