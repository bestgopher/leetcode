#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut row = 0u64;
        let mut col = 0u64;

        for e in &indices {
            let ri = e[0] as u64;
            let ci = e[1] as u64;
            row ^= 1u64 << ri;
            col ^= 1u64 << ci;
        }

        let cx = row.count_ones() as i32;
        let cy = col.count_ones() as i32;
        cx * (n - cy) + cy * (m - cx)
    }
}
