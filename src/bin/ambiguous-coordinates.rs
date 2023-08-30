#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s = &s[1..s.len() - 1];
        let mut result = vec![];
        for i in 0..s.len() {
            let s1 = Self::get_num(&s[..i]);
            let s2 = Self::get_num(&s[i..]);

            for j in s1.iter() {
                for k in s2.iter() {
                    result.push(format!("({j}, {k})"));
                }
            }
        }

        result
    }

    fn get_num(s: &str) -> Vec<String> {
        let mut r = vec![];
        if s.is_empty() {
            return r;
        }

        if s.as_bytes()[0] != b'0' || s.len() == 1 {
            r.push(String::from(s));
        }

        for i in 1..s.len() {
            if Self::is_valid_1(&s[..i]) && Self::is_valid_2(&s[i..]) {
                r.push(format!("{}.{}", &s[..i], &s[i..]))
            }
        }

        r
    }

    fn is_valid_1(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        if s.len() == 1 {
            return true;
        }

        if s.as_bytes()[0] == b'0' {
            return false;
        }

        true
    }

    fn is_valid_2(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        if s.as_bytes()[s.len() - 1] == b'0' {
            return false;
        }

        true
    }
}
