#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut current_capacity = capacity;
        let mut result = 0;
        for i in 0..plants.len() {
            result += 1;
            if current_capacity < plants[i] {
                current_capacity = capacity;
                result += i as i32 * 2;
            }

            current_capacity -= plants[i];
        }

        result
    }
}
