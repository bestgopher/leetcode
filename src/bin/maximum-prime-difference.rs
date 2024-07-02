#![allow(dead_code, unused, unused_variables, non_snake_case)]

extern crate core;

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let first = Self::prime(nums.iter());
        let last = nums.len() - 1 - Self::prime(nums.iter().rev());

        (last - first) as i32
    }

    fn prime<'a, T: Iterator<Item = &'a i32>>(iter: T) -> usize {
        fn is_prime(num: i32) -> bool {
            if num == 1 {
                return false;
            }

            for i in 2..num / 2 + 1 {
                if num % i == 0 {
                    return false;
                }
            }

            true
        }
        let mut i = 0;
        for &v in iter {
            if is_prime(v) {
                return i;
            }
            i += 1;
        }

        0
    }
}
