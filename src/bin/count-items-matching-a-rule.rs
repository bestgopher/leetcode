#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        items
            .into_iter()
            .filter(|x| match rule_key.as_str() {
                "type" => x[0] == rule_value,
                "color" => x[1] == rule_value,
                "name" => x[2] == rule_value,
                _ => unreachable!(),
            })
            .count() as i32
    }
}
