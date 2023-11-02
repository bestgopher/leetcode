#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut v = vec![0; 10];

        for i in (0..rings.len()).filter(|x| x % 2 == 1) {
            let x: u8 = match rings.as_bytes()[i - 1] {
                b'R' => 0b001,
                b'G' => 0b010,
                b'B' => 0b100,
                _ => unreachable!(),
            };

            v[(rings.as_bytes()[i] - b'0') as usize] |= x;
        }

        v.into_iter().filter(|x| *x & 0b111u8 == 0b111u8).count() as i32
    }
}
