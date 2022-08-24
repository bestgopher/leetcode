#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn str_to_int(str: String) -> i32 {
        let mut r = 0i32;
        let mut start = false; // 是否开始遍历
        let mut sign = 1; // 符号位

        for &i in str.as_bytes() {
            match i {
                b'-' | b'+' => {
                    if start {
                        break;
                    } else {
                        sign = if i == b'-' { -1 } else { 1 };
                        start = true;
                    }
                }
                b'0'..=b'9' => {
                    if !start {
                        start = true;
                    }

                    let (r1, is_overflow) = r.overflowing_mul(10);

                    if is_overflow {
                        if sign == 1 {
                            r = std::i32::MAX;
                        } else {
                            r = std::i32::MIN;
                        }
                        break;
                    }

                    let (r1, is_overflow) = r.overflowing_add((i - b'0') as i32 * sign);

                    if is_overflow {
                        if sign == 1 {
                            r = std::i32::MAX;
                        } else {
                            r = std::i32::MIN;
                        }
                        break;
                    }

                    r = r1;
                }
                b' ' => {
                    if start {
                        break;
                    }
                }
                _ => break,
            }
        }

        r
    }
}
