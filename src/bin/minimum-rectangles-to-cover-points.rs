#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut points = points;
        points.sort_by(|x, y| x[0].cmp(&y[0]));
        let mut result = 0;
        let mut index = 0;
        for i in 0..points.len() {
            if points[i][0] - points[index][0] > w {
                result += 1;
                index = i;
            }
        }

        result + 1
    }
}
