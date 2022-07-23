#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{:?}", Solution::get_row(6));
}

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut v = vec![0; row_index as usize + 1];

        for i in 0..row_index + 1 {
            v[i as usize] = 1;
            let mut x = v[0];
            for j in 1..i {
                let y = v[j as usize];
                v[j as usize] += x;
                x = y;
            }
        }
        v
    }
}
