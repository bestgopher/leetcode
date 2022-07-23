#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (mut sum1, mut sum2) = (0, 0);

        let mut start1 = start as usize;
        loop {
            if start1 == destination as usize {
                break;
            }
            if start1 == distance.len() - 1 {
                sum1 += distance[start1];
                start1 = 0;
            } else {
                sum1 += distance[start1];
                start1 += 1;
            }
        }

        let mut start2 = start as usize;
        loop {
            if start2 == destination as usize {
                break;
            }

            if start2 == 0 {
                sum2 += distance[distance.len() - 1];
                start2 = distance.len() - 1;
            } else {
                sum2 += distance[start2 - 1];
                start2 -= 1;
            }
        }

        if sum1 < sum2 {
            return sum1;
        }
        sum2
    }
}
