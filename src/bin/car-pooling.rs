#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut v = vec![0; 1002];

        for i in trips {
            v[i[1] as usize + 1] += i[0];
            v[i[2] as usize + 1] -= i[0];
        }

        for i in 1..v.len() {
            v[i] = v[i] + v[i - 1];
            if v[i] > capacity {
                return false;
            }
        }

        true
    }
}
