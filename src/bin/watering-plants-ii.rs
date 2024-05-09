#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut result = 0;
        let (mut a, mut b) = (capacity_a, capacity_b);
        let (mut i, mut j) = (0, plants.len() - 1);
        while i <= j {
            if i == j {
                if a < plants[i] && b < plants[i] {
                    result += 1;
                }
                break;
            } else {
                if a < plants[i] {
                    a = capacity_a;
                    result += 1;
                }

                if b < plants[j] {
                    b = capacity_b;
                    result += 1;
                }

                a -= plants[i];
                b -= plants[j];
                i += 1;
                j -= 1;
            }
        }

        result
    }
}
