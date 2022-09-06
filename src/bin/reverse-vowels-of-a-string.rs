#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s;
        let mut bytes = unsafe { s.as_bytes_mut() };

        let (mut start, mut end) = (0, bytes.len() - 1);

        while start < end {
            match bytes[start] {
                b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => {}
                _ => {
                    start += 1;
                    continue;
                }
            }

            match bytes[end] {
                b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => {}
                _ => {
                    end -= 1;
                    continue;
                }
            }

            bytes.swap(start, end);

            start += 1;
            end -= 1;
        }

        s
    }
}
