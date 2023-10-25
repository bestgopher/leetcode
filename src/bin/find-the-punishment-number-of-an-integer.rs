#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        (1..=n).into_iter().filter(Self::is).map(|x| x * x).sum()
    }

    pub fn is(n: &i32) -> bool {
        fn s(mut num: i32, except: i32) -> bool {
            if num == except {
                return true;
            }

            if num == 0 {
                return false;
            }
            let mut flag = 10;
            while num % flag != num {
                if s(num / flag, except - (num % flag)) {
                    return true;
                }

                flag *= 10;
            }

            false
        }

        s(n * n, *n)
    }
}
