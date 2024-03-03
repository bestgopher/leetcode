#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
        7
    );

    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
        5
    );
}

struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        (1..points.len())
            .map(|x| Self::distance(&points[x - 1], &points[x]))
            .sum()
    }

    /// 实际就是x[0]-y[0]与x[1]-y[1]的最大值
    fn distance(x: &[i32], y: &[i32]) -> i32 {
        (x[0] - y[0]).abs().max((x[1] - y[1]).abs())
    }
}
