#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(3, Solution::min_flips(2, 6, 5));
}

struct Solution;

impl Solution {
    pub fn min_flips1(a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;

        for i in 0..32 {
            let a1 = ((a as u32) << (31 - i)) >> 31;
            let b1 = ((b as u32) << (31 - i)) >> 31;
            let c1 = ((c as u32) << (31 - i)) >> 31;

            if c1 == 0 {
                if (a1 == 1 && b1 == 0) || (a1 == 0 && b1 == 1) {
                    count += 1;
                } else if a1 == 1 && b1 == 1 {
                    count += 2;
                }
            } else {
                if a1 == 0 && b1 == 0 {
                    count += 1;
                }
            }
        }
        count
    }

    // 任何数 &1 都只会剩下最后位的数字
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;

        for i in 0..32 {
            let a1 = (a >> i) & 1;
            let b1 = (b >> i) & 1;
            let c1 = (c >> i) & 1;

            if c1 == 0 {
                if (a1 == 1 && b1 == 0) || (a1 == 0 && b1 == 1) {
                    count += 1;
                } else if a1 == 1 && b1 == 1 {
                    count += 2;
                }
            } else {
                if a1 == 0 && b1 == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}
