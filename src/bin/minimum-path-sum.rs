#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut r = vec![vec![0; grid.len()]; grid[0].len()];
        Self::scan(&mut r, &grid, 0, 0);
        r[0][0]
    }

    fn scan(r: &mut Vec<Vec<i32>>, grid: &Vec<Vec<i32>>, index1: usize, index2: usize) {
        let (s1, s2) = (grid.len(), grid[0].len());
        if index1 == s1 - 1 && index2 == s2 - 1 {
            r[index1][index2] = grid[index1][index2];
            return;
        }

        if index1 >= s1 || index2 >= s2 || r[index1][index2] != 0 {
            return;
        }

        r[index1][index2] = grid[index1][index2];

        Self::scan(r, grid, index1 + 1, index2);
        Self::scan(r, grid, index1, index2 + 1);

        if index1 + 1 < s1 && index2 + 1 < s2 {
            r[index1][index2] += r[index1 + 1][index2].min(r[index1][index2 + 1]);
        } else if index1 + 1 >= s1 {
            r[index1][index2] += r[index1][index2 + 1];
        } else {
            r[index1][index2] += r[index1 + 1][index2];
        }
    }
}
