#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result = vec![];
        let mut n = text.split(' ');
        let mut f = n.next().unwrap();
        let mut is_result = false;
        while let Some(x) = n.next() {
            if is_result {
                result.push(x.to_string());
            }

            is_result = f == first && x == second;
            f = x;
        }

        result
    }
}
