#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut r = arr.len();

        let mut hash: std::collections::HashMap<i32, (usize, usize)> = Default::default();
        // s1存放每行的个数
        // s2存放没列的个数
        let (mut s1, mut s2) = (vec![0usize; mat.len()], vec![0usize; mat[0].len()]);

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                hash.insert(mat[i][j], (i, j));
            }
        }

        for i in 0..arr.len() {
            let (x, y) = hash[&arr[i]];

            s1[x] += 1usize;
            s2[y] += 1usize;

            if s1[x] == mat[0].len() || s2[y] == mat.len() {
                return i as i32;
            }
        }

        unreachable!()
    }
}
