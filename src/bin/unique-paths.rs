#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(28, Solution::unique_paths(3, 7));
}

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut v = vec![vec![0; n as usize]; m as usize];

        Self::calc(&mut v, 0, 0);

        v[0][0]
    }

    fn calc(array: &mut Vec<Vec<i32>>, m: usize, n: usize) {
        let s1 = array.len() - 1;
        let s2 = array[0].len() - 1;

        if m == s1 && n == s2 {
            array[s1][s2] = 1;
            return;
        }

        if array[m][n] != 0 {
            return;
        }

        if m == s1 {
            if array[m][n + 1] == 0 {
                Self::calc(array, m, n + 1);
            }
            array[m][n] = array[m][n + 1];
        } else if n == s2 {
            if array[m + 1][n] == 0 {
                Self::calc(array, m + 1, n);
            }
            array[m][n] = array[m + 1][n];
        } else {
            if array[m + 1][n] == 0 {
                Self::calc(array, m + 1, n);
            }

            if array[m][n + 1] == 0 {
                Self::calc(array, m, n + 1);
            }

            array[m][n] = array[m + 1][n] + array[m][n + 1];
        }
    }
}
