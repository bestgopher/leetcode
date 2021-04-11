fn main() {}

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut s = x as f64;
        let mut x1 = x as f64;

        while (s - x1 * x1).abs() > 0.1 {
            x1 = (s / x1 + x1) / 2f64;
        }

        x1 as i32
    }
}
