#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    // 从四个边界开始寻找，找到陆地变成水
    // 然后再次遍历，为陆地的地方即为岛屿？
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid[0].len() {
            Self::bfs((0, i), &mut grid);
        }

        for i in 0..grid[0].len() {
            Self::bfs((grid.len() - 1, i), &mut grid);
        }

        for i in 0..grid.len() {
            Self::bfs((i, 0), &mut grid);
        }

        for i in 0..grid.len() {
            Self::bfs((i, grid[0].len() - 1), &mut grid);
        }

        let mut r = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    r += 1;
                    Self::bfs((i, j), &mut grid);
                }
            }
        }

        r
    }

    fn bfs(start: (usize, usize), grid: &mut Vec<Vec<i32>>) {
        if start.0 > grid.len() - 1 {
            return;
        }

        if start.1 > grid[0].len() - 1 {
            return;
        }

        if grid[start.0][start.1] == 1 {
            return;
        } else {
            grid[start.0][start.1] = 1;
        }

        if start.0 > 0 {
            Self::bfs((start.0 - 1, start.1), grid);
        }

        if start.1 > 0 {
            Self::bfs((start.0, start.1 - 1), grid);
        }

        Self::bfs((start.0 + 1, start.1), grid);
        Self::bfs((start.0, start.1 + 1), grid);
    }
}
