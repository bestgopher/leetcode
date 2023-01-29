#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::force_largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
        10
    );
    assert_eq!(Solution::force_largest_rectangle_area(vec![2, 4]), 4);
    assert_eq!(Solution::force_largest_rectangle_area(vec![1, 1]), 2);
    assert_eq!(Solution::force_largest_rectangle_area(vec![2, 1, 2]), 3);
    assert_eq!(Solution::force_largest_rectangle_area(vec![5, 4, 1, 2]), 8);

    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1]), 2);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
    assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 1, 2]), 8);
    assert_eq!(Solution::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]), 6);
    assert_eq!(
        Solution::largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0]),
        20
    );
}

struct Solution;

impl Solution {
    /// 暴力解法
    pub fn force_largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut ans = 0;

        for i in 0..heights.len() {
            let mut left = i;

            for j in (0..i).rev() {
                if heights[j] >= heights[i] {
                    left = j;
                } else {
                    break;
                }
            }

            let mut right = i;

            for j in i..heights.len() {
                if heights[j] >= heights[i] {
                    right = j;
                } else {
                    break;
                }
            }

            ans = ans.max((right - left + 1) as i32 * heights[i])
        }

        ans
    }

    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        if len == 1 {
            return heights[0];
        }

        let mut stack: Vec<usize> = vec![];
        let mut ans = 0;

        // 循环便利整个heights
        for i in 0..len {
            while !stack.is_empty() && heights[stack[stack.len() - 1]] > heights[i] {
                let h = stack.pop().unwrap();
                if !stack.is_empty() {
                    ans = ans.max(heights[h] * (i - *stack.last().unwrap_or(&0) - 1) as i32);
                } else {
                    ans = ans.max(heights[h] * (i - *stack.last().unwrap_or(&0)) as i32);
                }
            }

            stack.push(i);
        }

        let last = *stack.last().unwrap_or(&0);

        while !stack.is_empty() {
            let h = stack.pop().unwrap();
            if stack.is_empty() {
                ans = ans.max(heights[h] * len as i32);
            } else {
                ans = ans.max(heights[h] * (last - *stack.last().unwrap_or(&0)) as i32);
            }
        }

        ans
    }
}
