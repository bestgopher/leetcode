#![allow(dead_code, unused, unused_variables)]

fn main() {
    let a = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];
    let b = Solution::diagonal_sort(a);
    println!("{:?}", b);

    let a = vec![
        vec![11, 25, 66, 1, 69, 7],
        vec![23, 55, 17, 45, 15, 52],
        vec![75, 31, 36, 44, 58, 8],
        vec![22, 27, 33, 25, 68, 4],
        vec![84, 28, 14, 11, 5, 50],
    ];
    let b = Solution::diagonal_sort(a);
    println!("{:?}", b);
}

struct Solution;

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if mat.is_empty() {
            return mat;
        }

        for i in 0..mat[0].len() {
            Self::bubble_sort(&mut mat, 0, i);
        }

        for i in 1..mat.len() {
            Self::bubble_sort(&mut mat, i, 0);
        }

        mat
    }

    pub fn bubble_sort(mat: &mut Vec<Vec<i32>>, x: usize, y: usize) {
        let (mut l1, mut l2) = (mat.len() - 1, mat[0].len() - 1);
        while l1 > x && l2 > y {
            let (mut x1, mut y1) = (x + 1, y + 1);

            while x1 <= l1 && y1 <= l2 {
                if mat[x1][y1] < mat[x1 - 1][y1 - 1] {
                    let a = mat[x1][y1];
                    mat[x1][y1] = mat[x1 - 1][y1 - 1];
                    mat[x1 - 1][y1 - 1] = a;
                }
                x1 += 1;
                y1 += 1;
            }

            l1 -= 1;
            l2 -= 1;
        }
    }
}
