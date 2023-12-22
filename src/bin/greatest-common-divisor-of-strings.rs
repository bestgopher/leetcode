#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut str1_ = str1.clone();
        let mut str2_ = str2.clone();

        let mut result = String::new();
        let mut i = 1;

        while i <= str1.len() && i <= str2.len() {
            if (i > str1.len() || str1.len() % i == 0) && (i > str2.len() || str2.len() % i == 0) {
                let s1 = Self::gdc(str1.as_bytes(), i);
                let s2 = Self::gdc(str2.as_bytes(), i);

                if s1 == s2 {
                    result = String::from_utf8_lossy(s1).to_string()
                }
            }

            i += 1;
        }

        result
    }

    fn gdc(s: &[u8], mut i: usize) -> &[u8] {
        if i > s.len() {
            return &s[0..0];
        }

        let mut start = i;
        while start <= s.len() {
            if s[start - i..start] != s[0..i] {
                return &s[0..0];
            }

            start += i;
        }

        &s[0..i]
    }
}
