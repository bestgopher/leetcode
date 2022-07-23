#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut x = vec![vec![]; triangle.len()];
        Self::f(&triangle, &mut x, 0, 0);
        x[0][0]
    }

    /// index1: 二维数组的下标
    /// index2: 行的下标
    fn f(v: &Vec<Vec<i32>>, x: &mut Vec<Vec<i32>>, index1: usize, index2: usize) {
        if v.len() - 1 == index1 {
            x[index1] = v[index1].clone();
            return;
        }

        if index2 == 0 {
            x[index1] = vec![-1; v[index1].len()];
        }

        if x[index1][index2] != -1 {
            return;
        }

        Self::f(v, x, index1 + 1, index2);
        Self::f(v, x, index1 + 1, index2 + 1);

        x[index1][index2] = v[index1][index2] + x[index1 + 1][index2].min(x[index1 + 1][index2 + 1])
    }
}
