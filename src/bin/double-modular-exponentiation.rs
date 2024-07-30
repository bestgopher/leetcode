#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        variables
            .into_iter()
            .enumerate()
            .filter_map(|(x, y)| {
                if (Self::pow(Self::pow(y[0], y[1], 10), y[2], y[3])) == target {
                    Some(x as i32)
                } else {
                    None
                }
            })
            .collect()
    }

    fn pow(mut x: i32, mut y: i32, z: i32) -> i32 {
        let mut result = 1;
        while y > 0 {
            if y % 2 > 0 {
                result = result * x % z;
            }

            x = x * x % z;
            y /= 2;
        }

        result
    }
}
