use serde::export::Option::Some;

fn main() {}

struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v: Vec<char> = vec![];

        for i in s.chars() {
            if let Some(s) = v.last() {
                if s == &i {
                    v.pop();
                    continue;
                }
            }

            v.push(i);
        }

        v.iter().collect()
    }
}
