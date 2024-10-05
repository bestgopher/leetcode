#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut v = [0; 5];
        for i in text.bytes() {
            match i {
                b'b' => v[0] += 1,
                b'a' => v[1] += 1,
                b'l' => v[2] += 1,
                b'o' => v[3] += 1,
                b'n' => v[4] += 1,
                _ => {}
            }
        }

        v[0].min(v[1]).min(v[2] / 2).min(v[3] / 2).min(v[4])
    }
}
