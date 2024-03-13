#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::maximum_odd_binary_number("0101".to_string()),
        "1001".to_string()
    );

    assert_eq!(
        Solution::maximum_odd_binary_number("010".to_string()),
        "001".to_string()
    );
}

struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut l = s.len();
        let m = s.bytes().into_iter().filter(|x| *x == b'1').count();
        let mut s = String::with_capacity(l);
        for i in 0..l - 1 {
            if i < m - 1 {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.push('1');
        s
    }
}
