#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    // 先找出多余的括号，然后在移除多余的括号
    pub fn min_remove_to_make_valid(s: String) -> String {
        let (mut v1, mut v2) = (0, 0); // v1：左括号多余个数，v2:右括号多余个数
        let (mut v3, mut v4) = (0, 0); // v1：左括号的总个数，v2:右括号的总个数
        for &i in s.as_bytes().into_iter() {
            if i == b'(' {
                v1 += 1;
                v3 += 1;
            } else if i == b')' {
                if v1 != 0 {
                    v1 -= 1;
                } else {
                    v2 += 1;
                }
                v4 += 1;
            }
        }

        let mut s1 = String::with_capacity(s.as_bytes().len() - (v1 + v2) as usize);
        let mut v5 = 0; // 已有的左括号的个数
        for i in s.chars().into_iter() {
            if i == '(' {
                if v3 > v1 {
                    s1.push(i);
                    v3 -= 1;
                    v5 += 1;
                }
            } else if i == ')' {
                if v4 > v2 && v5 != 0 {
                    s1.push(i);
                    v4 -= 1;
                    v5 -= 1;
                }
            } else {
                s1.push(i);
            }
        }

        s1
    }
}
