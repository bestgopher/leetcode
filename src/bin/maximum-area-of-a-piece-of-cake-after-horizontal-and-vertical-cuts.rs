#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();
        let mut r = 0;

        let horizontal_max = Self::get_max(h, horizontal_cuts);
        let vertical_max = Self::get_max(w, vertical_cuts);

        ((horizontal_max as i64) * (vertical_max as i64) % (10i64.pow(9) + 7)) as i32
    }

    fn get_max(s: i32, cuts: Vec<i32>) -> i32 {
        let mut max = cuts[0].max(s - cuts[cuts.len() - 1]);

        if cuts.len() > 1 {
            for i in 1..cuts.len() {
                max = max.max(cuts[i] - cuts[i - 1]);
            }
        }

        max
    }
}
