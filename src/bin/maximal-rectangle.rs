#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        // 构造柱状图
        let mut v = vec![vec![]; matrix.len()];

        for i in 0..matrix.len() {
            v[i].push(0);
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '0' {
                    v[i].push(0);
                } else {
                    let value = if i == 0 { 1 } else { v[i - 1][j + 1] + 1 };
                    v[i].push(value);
                }
            }

            v[i].push(0);
        }

        let mut ans = 0;

        for v1 in v {
            let len = v1.len();
            let mut stack = vec![];

            for v2 in 0..len {
                while !stack.is_empty() && v1[stack[stack.len() - 1]] >= v1[v2] {
                    let top = stack.pop().unwrap();
                    ans = ans.max(v1[top] * (v2 - *stack.last().unwrap_or(&0) - 1) as i32);
                }

                stack.push(v2);
            }

            let last = *stack.last().unwrap_or(&0);

            while !stack.is_empty() {
                let top = stack.pop().unwrap();
                ans = ans.max(v1[top] * (last - *stack.last().unwrap_or(&0)) as i32);
            }
        }

        ans
    }
}
