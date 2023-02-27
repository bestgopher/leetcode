#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 100]), 100);
    assert_eq!(
        Solution::single_number(vec![-2, -2, 1, 1, 4, 1, 4, 4, -4, -2]),
        -4
    );
    assert_eq!(
        Solution::single_number(vec![
            43,
            16,
            45,
            89,
            45,
            -2147483648,
            45,
            2147483646,
            -2147483647,
            -2147483648,
            43,
            2147483647,
            -2147483646,
            -2147483648,
            89,
            -2147483646,
            89,
            -2147483646,
            -2147483647,
            2147483646,
            -2147483647,
            16,
            16,
            2147483646,
            43
        ]),
        2147483647
    );
}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut v = [0; std::mem::size_of::<i32>() * 8];

        for mut i in nums {
            for j in 0..32 {
                if (i >> j) & 1 == 1 {
                    v[j] = (v[j] + 1) % 3;
                }
            }
        }

        let mut ans = 0;
        for i in 0..v.len() {
            if v[i] == 1 {
                ans += (1 << i);
            }
        }

        ans
    }
}
