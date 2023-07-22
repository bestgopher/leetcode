#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut moneys = [0; 2]; // 第一个元素代表5的个数，第二个代表10的个数

        for i in bills {
            match i {
                5 => moneys[0] += 1,
                10 => {
                    if moneys[0] == 0 {
                        return false;
                    }

                    moneys[0] -= 1;
                    moneys[1] += 1;
                }
                20 => {
                    // 可以找1张5元+1张10元
                    // 也可以找3张5元
                    // 优先找10元
                    if moneys[0] >= 1 && moneys[1] >= 1 {
                        moneys[0] -= 1;
                        moneys[1] -= 1;
                    } else if moneys[0] >= 3 {
                        moneys[0] -= 3;
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        true
    }
}
