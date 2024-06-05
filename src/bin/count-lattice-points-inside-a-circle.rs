#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut max_x = -1;
        let mut max_y = -1;
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut result = 0;
        for i in circles.iter() {
            max_x = max_x.max(i[0] + i[2]);
            max_y = max_y.max(i[1] + i[2]);
            min_x = min_x.min(i[0] - i[2]);
            min_y = min_y.min(i[1] - i[2]);
        }

        for i in min_x..=max_x {
            for j in min_y..=max_y {
                for c in circles.iter() {
                    if (i - c[0]).pow(2) + (j - c[1]).pow(2) <= c[2].pow(2) {
                        result += 1;
                        break;
                    }
                }
            }
        }

        result
    }
}
