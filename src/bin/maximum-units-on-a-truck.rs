#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_unstable_by(|x, y| {
            use std::cmp::Ordering;
            match x[1].cmp(&y[1]) {
                Ordering::Equal => {}
                x => return x.reverse(),
            }

            x[0].cmp(&y[1]).reverse()
        });

        let mut r = 0;
        let mut truck_size = truck_size;

        for box_type in box_types {
            if box_type[0] >= truck_size {
                r += truck_size * box_type[1];
                break;
            } else {
                r += box_type[0] * box_type[1];
                truck_size -= box_type[0];
            }
        }

        r
    }
}
