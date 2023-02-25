#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        let mut start = 0;
        let mut pow = 1;
        for i in 1..=n {
            if i == pow {
                start = 0;
                pow <<= 1;
            }

            ans[i as usize] = ans[start] + 1;
            start += 1;
        }

        ans
    }
}
