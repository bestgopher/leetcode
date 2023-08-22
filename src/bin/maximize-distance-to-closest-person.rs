#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut last = -1;

        for i in 0..seats.len() {
            if seats[i] == 1 {
                if last == -1 {
                    s = s.max(i as i32);
                } else {
                    s = s.max((i as i32 - last) / 2);
                }
                last = i as i32;
            }
        }

        if seats[seats.len() - 1] == 0 {
            s = s.max(seats.len() as i32 - 1 - last);
        }

        s
    }
}
