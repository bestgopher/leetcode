#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// s1[i] != s2[i] 的个数d
    /// 如果个数为奇数，则永远不可能完成
    /// 如果个数为偶数，如果x和y的个数为偶数，则 d / 2
    /// 否则 d/2+1
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut x = 0;
        let mut c_x = 0;
        for i in 0..s1.len() {
            if s1.as_bytes()[i] != s2.as_bytes()[i] {
                x += 1;

                if s1.as_bytes()[i] == b'x' {
                    c_x += 1;
                }
            }
        }

        if x % 2 == 1 {
            -1
        } else {
            if c_x % 2 == 0 {
                x / 2
            } else {
                x / 2 + 1
            }
        }
    }
}
