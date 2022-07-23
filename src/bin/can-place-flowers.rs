#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut f = flowerbed;
        let mut n = n;

        for i in 0..f.len() {
            if i == 0 {
                if f[i] == 0 && f.get(i + 1).unwrap_or(&0) == &0 {
                    f[i] = 1;
                    n -= 1;
                }
            } else if i == f.len() - 1 {
                if f[i] == 0 && f.get(i - 1).unwrap_or(&0) == &0 {
                    f[i] = 1;
                    n -= 1;
                }
            } else {
                if f[i] == 0 && f[i - 1] == 0 && f[i + 1] == 0 {
                    f[i] = 1;
                    n -= 1;
                }
            }

            if n <= 0 {
                return true;
            }
        }

        false
    }
}
