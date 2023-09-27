#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        const A: i32 = (b'z' - b'a') as _;
        let mut result = vec![b'z'; n as _];
        let mut total = (A + 1) * n;
        for i in 0..n as _ {
            if total - k >= A {
                result[i] -= A as u8;
                total -= A;
            } else {
                result[i] -= (total - k) as u8;
                break;
            }
        }

        String::from_utf8(result).unwrap()
    }
}
