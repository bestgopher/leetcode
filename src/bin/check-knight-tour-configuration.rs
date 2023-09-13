#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            return false;
        }

        let (mut x, mut y) = (0, 0);
        let mut result = false;
        for i in 1..grid.len() * grid.len() {
            let (r1, x1, y1) = Self::check(&grid[..], x, y, i as i32);
            if !r1 {
                return false;
            }

            x = x1;
            y = y1;
        }

        true
    }

    pub fn check(grid: &[Vec<i32>], x: usize, y: usize, expect: i32) -> (bool, usize, usize) {
        if x > 1 {
            if y > 0 {
                if grid[x - 2][y - 1] == expect {
                    return (true, x - 2, y - 1);
                }
            }

            if y < grid.len() - 1 {
                if grid[x - 2][y + 1] == expect {
                    return (true, x - 2, y + 1);
                }
            }
        }

        if x < grid.len() - 2 {
            if y > 0 {
                if grid[x + 2][y - 1] == expect {
                    return (true, x + 2, y - 1);
                }
            }

            if y < grid.len() - 1 {
                if grid[x + 2][y + 1] == expect {
                    return (true, x + 2, y + 1);
                }
            }
        }

        if y > 1 {
            if x > 0 {
                if grid[x - 1][y - 2] == expect {
                    return (true, x - 1, y - 2);
                }
            }

            if x < grid.len() - 1 {
                if grid[x + 1][y - 2] == expect {
                    return (true, x + 1, y - 2);
                }
            }
        }

        if y < grid.len() - 2 {
            if x > 0 {
                if grid[x - 1][y + 2] == expect {
                    return (true, x - 1, y + 2);
                }
            }

            if x < grid.len() - 1 {
                if grid[x + 1][y + 2] == expect {
                    return (true, x + 1, y + 2);
                }
            }
        }

        (false, 0, 0)
    }
}
