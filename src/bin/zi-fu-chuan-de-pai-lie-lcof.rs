#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    println!("{:?}", Solution::permutation("abc".to_string()));
}

struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        Self::f(s.as_bytes())
            .into_iter()
            .map(|x| String::from_utf8(x).unwrap())
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect()
    }

    fn f(x: &[u8]) -> Vec<Vec<u8>> {
        if x.len() == 1 {
            return vec![vec![x[0]]];
        }
        let mut data = vec![];
        for i in 0..x.len() {
            let n = x[i];
            let mut v = Vec::with_capacity(x.len());
            v.extend(&x[0..i]);
            v.extend(&x[i + 1..]);

            let x1 = Self::f(&v[..]);
            for j in x1.iter() {
                let mut m = Vec::with_capacity(x.len());
                m.push(n);
                m.extend(j.into_iter());
                data.push(m);
            }
        }

        data
    }
}
