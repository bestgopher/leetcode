#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        const A: i32 = 10i32.pow(4);
        const B: i64 = 10i64.pow(9);
        let is_bulky = length >= A
            || width >= A
            || height >= A
            || (length as i64 * width as i64 * height as i64) >= B;
        let is_heavy = mass >= 100;

        if is_bulky && is_heavy {
            "Both"
        } else if is_bulky {
            "Bulky"
        } else if is_heavy {
            "Heavy"
        } else {
            "Neither"
        }
        .into()
    }
}
