#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 找到矩形边上的点到圆心的距离是否小于等于圆的半径
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let mut d = 0;
        if x_center < x1 || x_center > x2 {
            d += (x1 - x_center).pow(2).min((x2 - x_center).pow(2));
        }

        if y_center < y1 || y_center > y2 {
            d += (y1 - y_center).pow(2).min((y2 - y_center).pow(2));
        }

        d <= radius.pow(2)
    }
}
