#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut hash = std::collections::HashSet::new();
        let mut stack = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    1 => {
                        hash.insert((i, j));
                    }
                    2 => stack.push((i, j)),
                    _ => {}
                }
            }
        }

        let mut result = 0;
        while !stack.is_empty() {
            let mut new_stack = vec![];
            let len = hash.len();
            while let Some((x, y)) = stack.pop() {
                if x > 0 {
                    if hash.contains(&(x - 1, y)) {
                        new_stack.push((x - 1, y));
                        hash.remove(&(x - 1, y));
                    }
                }

                if y > 0 {
                    if hash.contains(&(x, y - 1)) {
                        new_stack.push((x, y - 1));
                        hash.remove(&(x, y - 1));
                    }
                }

                if hash.contains(&(x + 1, y)) {
                    new_stack.push((x + 1, y));
                    hash.remove(&(x + 1, y));
                }

                if hash.contains(&(x, y + 1)) {
                    new_stack.push((x, y + 1));
                    hash.remove(&(x, y + 1));
                }
            }
            if hash.len() != len {
                result += 1;
            }
            stack = new_stack;
        }

        if hash.is_empty() {
            result
        } else {
            -1
        }
    }
}
