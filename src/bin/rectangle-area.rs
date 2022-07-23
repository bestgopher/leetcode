#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let mut s = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);

        let (px1, py1) = (bx1.max(ax1), by1.max(ay1));
        let (px2, py2) = (bx2.min(ax2), by2.min(ay2));

        if px2 > px1 && py2 > py1 {
            s -= (px2 - px1) * (py2 - py1);
        }

        s
    }
}
