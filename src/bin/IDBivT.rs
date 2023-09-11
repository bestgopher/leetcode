#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        } else if n == 0 {
            return vec!["".to_string()];
        }
        let mut ans = vec![];
        for i in 0..n {
            let mut mid = vec![];
            for sub_string1 in Self::generate_parenthesis(i) {
                mid.push(format!("({sub_string1})"));
            }
            for m in mid {
                for sub_string2 in Self::generate_parenthesis(n - 1 - i) {
                    ans.push(format!("{m}{sub_string2}"));
                }
            }
        }
        ans
    }
}
