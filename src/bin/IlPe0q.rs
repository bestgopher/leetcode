#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    triangle[i][j] += triangle[i - 1][0];
                } else if j == triangle[i].len() - 1 {
                    triangle[i][j] += triangle[i - 1][j - 1];
                } else {
                    triangle[i][j] += triangle[i - 1][j - 1].min(triangle[i - 1][j]);
                }
            }
        }

        triangle[triangle.len() - 1]
            .iter()
            .map(|x| *x)
            .min()
            .unwrap()
    }
}
