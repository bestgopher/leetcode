#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dict = [0; 26];
        for (i, &v) in order.as_bytes().into_iter().enumerate() {
            dict[(v - b'a') as usize] = i
        }

        for i in 1..words.len() {
            let mut index = 0;
            loop {
                match (
                    words[i - 1].as_bytes().get(index),
                    words[i].as_bytes().get(index),
                ) {
                    (Some(&x), Some(&y)) => {
                        if dict[(x - b'a') as usize] > dict[(y - b'a') as usize] {
                            return false;
                        } else if dict[(x - b'a') as usize] < dict[(y - b'a') as usize] {
                            break;
                        }
                    }
                    (Some(&x), None) => return false,
                    _ => {
                        break;
                    }
                }

                index += 1;
            }
        }

        true
    }
}
