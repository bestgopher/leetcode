#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    /// 直线肯定经过两个正方形的中心点，所以只能有一条
    /// 当两个正方形重合时，就有无数条, 取平行于y轴的线
    pub fn cut_squares(square1: Vec<i32>, square2: Vec<i32>) -> Vec<f64> {
        // 两个正方形的中心
        let sq1_center = (
            square1[0] as f64 + (square1[2] as f64) / 2f64,
            (square1[1] as f64) + (square1[2] as f64) / 2f64,
        );
        let sq2_center = (
            square2[0] as f64 + (square2[2] as f64) / 2f64,
            (square2[1] as f64) + (square2[2] as f64) / 2f64,
        );

        let mut result = vec![];
        // sq1_center.0 == sq2_center.1时斜率无穷大
        if sq1_center.0 == sq2_center.1 {
            let x1 = sq1_center.0;
            let y1 = (sq1_center.1 - square1[2] as f64 / 2f64)
                .min(sq2_center.1 - square2[2] as f64 / 2f64);
            let x2 = sq1_center.0;
            let y2 = (sq1_center.1 + square1[2] as f64 / 2f64)
                .max(sq2_center.1 + square2[2] as f64 / 2f64);
            result = vec![x1, y1, x2, y2];
        } else {
            // 求y = kx+b
            // 求斜率k。斜率大于1/2则坐落于上下边，否则在左右边
            let k = (sq1_center.1 - sq2_center.1) / (sq1_center.0 - sq2_center.0);
            // 求b
            let b = sq1_center.1 - k * sq1_center.0;

            let f = |x: f64| k * x + b;
            let f1 = |y: f64| (y - b) / k;

            // -1<k<1,端点在左右边上，否则在上下边上
            if k < 1f64 && k > -1f64 {
                let x1 = (square1[0] as f64).min(square2[0] as f64);
                let y1 = f(x1);
                let x2 = ((square1[0] + square1[2]) as f64).max((square2[0] + square2[2]) as f64);
                let y2 = f(x2);
                result = vec![x1, y1, x2, y2];
            } else {
                let y1 = (square1[1] as f64).min(square2[1] as f64);
                let x1 = f1(y1);
                let y2 = ((square1[1] + square1[2]) as f64).max((square2[1] + square2[2]) as f64);
                let x2 = f1(y2);
                result = vec![x1, y1, x2, y2];
            }

            if result[0] != result[2] {
                if result[0] > result[2] {
                    result.swap(0, 2);
                    result.swap(1, 3);
                }
            } else {
                if result[1] > result[3] {
                    result.swap(0, 2);
                    result.swap(1, 3);
                }
            }
        }
        result
    }
}
