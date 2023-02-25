#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{}", Solution::add_binary("11".into(), "10".into()));
    println!("{}", Solution::add_binary("1010".into(), "1011".into()));
}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ans = vec![0; a.len().max(b.len()) + 1];
        let l = ans.len();
        let mut sign = b'0'; // 符号位
        let (a, b) = (a.as_bytes(), b.as_bytes());

        let mut i = 1;

        while i <= a.len() || i <= b.len() {
            if i <= a.len() && i <= b.len() {
                match (a[a.len() - i], b[b.len() - i], sign) {
                    (b'0', b'0', s) => {
                        ans[l - i] = s;
                        sign = b'0';
                    }
                    (b'1', b'0', s) => {
                        if s == b'1' {
                            ans[l - i] = b'0';
                            sign = b'1';
                        } else {
                            ans[l - i] = b'1';
                            sign = b'0';
                        }
                    }
                    (b'0', b'1', s) => {
                        if s == b'1' {
                            ans[l - i] = b'0';
                            sign = b'1';
                        } else {
                            ans[l - i] = b'1';
                            sign = b'0';
                        }
                    }
                    (b'1', b'1', s) => {
                        ans[l - i] = s;
                        sign = b'1';
                    }
                    _ => unreachable!(),
                }
            } else if i <= a.len() {
                match (a[a.len() - i], sign) {
                    (b'0', s) => {
                        ans[l - i] = s;
                        sign = b'0';
                    }
                    (b'1', s) => {
                        if s == b'0' {
                            ans[l - i] = b'1';
                            sign = b'0';
                        } else {
                            ans[l - i] = b'0';
                            sign = b'1';
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                match (b[b.len() - i], sign) {
                    (b'0', s) => {
                        ans[l - i] = s;
                        sign = b'0';
                    }
                    (b'1', s) => {
                        if s == b'0' {
                            ans[l - i] = b'1';
                            sign = b'0';
                        } else {
                            ans[l - i] = b'0';
                            sign = b'1';
                        }
                    }
                    _ => unreachable!(),
                }
            }

            i += 1;
        }

        if sign == b'1' {
            ans[l - i] = b'1';
        }

        if ans[0] == b'1' {
            String::from_utf8_lossy(&ans).to_string()
        } else {
            String::from_utf8_lossy(&ans[1..]).to_string()
        }
    }
}
