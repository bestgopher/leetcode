#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

        for i in 1..=num_rows {
            let mut r = Vec::with_capacity(i as usize);
            for j in 0..i {
                if j == 0 || j == i - 1 {
                    r.push(1);
                } else {
                    r.push(
                        result[i as usize - 2][j as usize - 1] + result[i as usize - 2][j as usize],
                    );
                }
            }

            result.push(r);
        }

        result
    }
}
