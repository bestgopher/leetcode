#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::num_trees(3));
    println!("{}", Solution::num_trees(4));
    println!("{}", Solution::num_trees(19));
}

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut v = vec![0; n as usize + 1];
        v[0] = 1;
        v[1] = 1;

        for i in 2..=n as usize {
            for j in 1..=i {
                v[i] += v[j - 1] * v[i - j];
            }
        }

        v[n as usize]
    }
}
