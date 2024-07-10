#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn master_mind(solution: String, guess: String) -> Vec<i32> {
        let mut v = vec![0; 4];
        for &i in solution.as_bytes() {
            match i {
                b'R' => v[0] += 1,
                b'Y' => v[1] += 1,
                b'G' => v[2] += 1,
                b'B' => v[3] += 1,
                _ => {}
            }
        }
        let mut right1 = 0;
        let mut right2 = 0;

        for i in 0..4 {
            if solution.as_bytes()[i] == guess.as_bytes()[i] {
                right1 += 1;
            }

            match guess.as_bytes()[i] {
                b'R' => {
                    if v[0] > 0 {
                        v[0] -= 1;
                        right2 += 1;
                    }
                }
                b'Y' => {
                    if v[1] > 0 {
                        v[1] -= 1;
                        right2 += 1;
                    }
                }
                b'G' => {
                    if v[2] > 0 {
                        v[2] -= 1;
                        right2 += 1;
                    }
                }
                b'B' => {
                    if v[3] > 0 {
                        v[3] -= 1;
                        right2 += 1;
                    }
                }
                _ => {}
            }
        }

        vec![right1, right2 - right1]
    }
}
