#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let mut v = vec![];
        for i in matrix.iter() {
            let mut s = 0;
            for j in 0..i.len() {
                s |= i[j] << (i.len() - j - 1) as i32;
            }

            v.push(s);
        }

        let mut result = 0;
        let mut s: i32 = (0..matrix[0].len()).fold(1, |x, y| x | (1 << y));

        for i in 0..=s {
            if i.count_ones() as i32 == num_select {
                let mut r = 0;
                for &v in v.iter() {
                    if v & i == v {
                        r += 1;
                    }
                }
                result = result.max(r);
            }
        }

        result
    }
}
