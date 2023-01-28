#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(
        Solution::trap_stack(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
        6
    );
}

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_left = vec![0; height.len()]; // 左边的最大值

        for (i, &v) in height.iter().enumerate() {
            if i == 0 {
                max_left[i] = v;
            } else {
                max_left[i] = v.max(max_left[i - 1]);
            }
        }

        let mut max_right = vec![0; height.len()];

        for (i, &v) in height.iter().rev().enumerate() {
            let index = max_right.len() - i - 1;
            if i == 0 {
                max_right[index] = v;
            } else {
                max_right[index] = v.max(max_right[index + 1]);
            }
        }

        let mut ans = 0;

        for (i, &v) in height.iter().enumerate() {
            let min = max_left[i].min(max_right[i]);
            if v < min {
                ans += min - v;
            }
        }

        ans
    }

    pub fn trap_stack(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack = vec![];

        for i in 0..height.len() {
            while !stack.is_empty() && height[stack[stack.len() - 1]] < height[i] {
                let top = stack.pop().unwrap();
                if stack.is_empty() {
                    break;
                }

                let &l = stack.last().unwrap();
                let h = height[i].min(height[l]) - height[top];
                ans += (i as i32 - l as i32 - 1) * h;
            }

            stack.push(i);
        }

        ans
    }
}
