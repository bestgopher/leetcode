#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 将绝对值去掉
    /// |arr1[i] - arr1[j]| + |arr2[i] - arr2[j]| + |i - j|
    //
    //  =  (arr1[i] + arr2[i] + i) - (arr1[j] + arr2[j] + j)
    //  =  (arr1[i] + arr2[i] - i) - (arr1[j] + arr2[j] - j)
    //  =  (arr1[i] - arr2[i] + i) - (arr1[j] - arr2[j] + j)
    //  =  (arr1[i] - arr2[i] - i) - (arr1[j] - arr2[j] - j)
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let (mut a_max, mut b_max, mut c_max, mut d_max) = (i32::MIN, i32::MIN, i32::MIN, i32::MIN);
        let (mut a_min, mut b_min, mut c_min, mut d_min) = (i32::MAX, i32::MAX, i32::MAX, i32::MAX);

        for i in 0..arr1.len() {
            let (x, y) = (arr1[i], arr2[i]);
            a_max = a_max.max(x + y + i as i32);
            a_min = a_min.min(x + y + i as i32);

            b_max = b_max.max(x + y - i as i32);
            b_min = b_min.min(x + y - i as i32);

            c_max = c_max.max(x - y + i as i32);
            c_min = c_min.min(x - y + i as i32);

            d_max = d_max.max(x - y - i as i32);
            d_min = d_min.min(x - y - i as i32);
        }

        (a_max - a_min)
            .max(b_max - b_min)
            .max(c_max - c_min)
            .max(d_max - d_min)
    }
}
