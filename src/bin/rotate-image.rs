#![allow(dead_code, unused, unused_variables)]

fn main() {
    let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    Solution::rotate(&mut v);
    println!("{:?}", v);
    println!("{}", "[[7, 4, 1], [8, 5, 2], [9, 6, 3]]");
}

struct Solution;

impl Solution {
    pub fn rotate1(matrix: &mut Vec<Vec<i32>>) {
        let l = matrix.len();
        for i in 0..=l / 2 {
            for j in i..l - i - 1 {
                let mut next = (i, j);
                for k in 0..=3 {
                    println!("{:?}", next);

                    if k == 0 {
                        next = (i, j);
                    } else if k == 1 {
                        next = (j, l - i - 1);
                    } else if k == 2 {
                        next = (l - i - 1, l - j - 1);
                    } else {
                        next = (l - j - 1, i);
                    }

                    let last = matrix[next.0][next.1];
                    matrix[next.0][next.1] = matrix[i][j];
                    matrix[i][j] = last;
                }
            }
        }
    }

    /// 先沿着右上左下对角线折叠，
    /// 然后再沿着中线反转即可实现
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let l = matrix.len();
        for i in 0..l {
            for j in 0..l - i - 1 {
                let x = matrix[i][j];
                matrix[i][j] = matrix[l - j - 1][l - i - 1];
                matrix[l - j - 1][l - i - 1] = x;
            }
        }

        for i in 0..=l / 2 {
            for j in 0..l {
                let x = matrix[i][j];
                matrix[i][j] = matrix[l - i - 1][j];
                matrix[l - i - 1][j] = x;
            }
        }
    }
}
