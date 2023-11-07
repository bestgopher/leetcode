#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut max = 0;
        let mut zero_num = 0;
        let mut one_num = 0;
        for &i in s.as_bytes() {
            match i {
                b'0' => {
                    if one_num != 0 {
                        one_num = 0;
                        zero_num = 0;
                    }
                    zero_num += 1;
                }
                b'1' => {
                    if one_num < zero_num {
                        one_num += 1;
                    } else {
                        zero_num = 0;
                    }

                    max = max.max(one_num.min(zero_num) * 2);
                }
                _ => unreachable!(),
            }
        }

        max
    }
}
