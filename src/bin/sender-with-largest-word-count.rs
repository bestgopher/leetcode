#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut h = std::collections::HashMap::new();

        for (i, v) in messages.iter().enumerate() {
            let n = v.split(' ').count();
            h.entry(&senders[i])
                .and_modify(|x| *x += n as i32)
                .or_insert(n as i32);
        }
        let mut r = &String::new();
        let mut i = 0;
        for (n, v) in h {
            if v > i {
                r = n;
                i = v;
            } else if v == i {
                r = r.max(n);
            }
        }

        r.clone()
    }
}
