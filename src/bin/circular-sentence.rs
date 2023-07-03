#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let bytes = sentence.as_bytes();
        if bytes[0] != bytes[bytes.len() - 1] {
            return false;
        }

        let (mut current, mut flag) = (bytes[0], false);
        for i in sentence.bytes() {
            if i == b' ' {
                flag = true;
            } else {
                if flag {
                    if i != current {
                        return false;
                    }
                    flag = false;
                }

                current = i;
            }
        }

        true
    }
}
