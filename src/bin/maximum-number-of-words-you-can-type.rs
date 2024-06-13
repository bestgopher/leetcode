#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut s = [0; 26];
        for &i in broken_letters.as_bytes() {
            s[(i - b'a') as usize] = 1;
        }

        let mut res = 0;
        'l: for i in text.split(' ') {
            for &j in i.as_bytes() {
                if s[(j - b'a') as usize] == 1 {
                    continue 'l;
                }
            }

            res += 1;
        }

        res
    }
}
