#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        let mut k = k;
        let mut r = 0;

        if k > num_ones {
            k -= num_ones;
            r += num_ones;
        } else {
            return k;
        }

        if k > num_zeros {
            k -= num_zeros;
        } else {
            return r;
        }

        r - k
    }
}
