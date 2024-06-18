#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let mut result = String::new();

        for mut s in sentence.split(' ') {
            if s.starts_with('$') {
                result.push('$');
                s = &s[1..];

                match s.parse::<i64>() {
                    Ok(i) => result.push_str(&format!(
                        "{:.2}",
                        i as f64 * (1.0 - discount as f64 / 100.0)
                    )),
                    _ => result.push_str(s),
                }
            } else {
                result.push_str(s);
            }

            result.push(' ');
        }

        result.pop();
        result
    }
}
