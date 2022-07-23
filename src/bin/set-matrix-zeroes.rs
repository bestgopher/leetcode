#![allow(dead_code, unused, unused_variables)]

fn main() {
    let mut x = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut x);
}

struct Solution;

impl Solution {
    /// 扫描。常规做法
    pub fn set_zeroes1(matrix: &mut Vec<Vec<i32>>) {
        let (mut v1, mut v2) = (vec![0; matrix.len()], vec![0; matrix[0].len()]);
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    v1[i] = 1;
                    v2[j] = 1;
                }
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if v1[i] == 1 || v2[j] == 1 {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    /// 常规空间的做法
    /// 标记做法：循环遍历每个元素，当元素为0时将这个元素对应的第一行的元素和第一列的元素都改为0
    /// 然后从(1,1)开始遍历
    /// 如果第一行或者第一列存在0，还要改变第一行和第一列
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut row, mut column) = (false, false);

        for i in 0..matrix.len() {
            if matrix[i][0] == 0 {
                column = true;
                break;
            }
        }

        for j in 0..matrix[0].len() {
            if matrix[0][j] == 0 {
                row = true;
                break;
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if row {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }

        if column {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
    }
}
