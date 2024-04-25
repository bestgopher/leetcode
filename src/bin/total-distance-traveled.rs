#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        let (mut main_tank, mut additional_tank) = (main_tank, additional_tank);
        let mut result = 0;

        while main_tank > 0 {
            if main_tank >= 5 {
                main_tank -= 5;
                result += 50;
                if additional_tank > 0 {
                    main_tank += 1;
                    additional_tank -= 1;
                }
            } else {
                result += main_tank * 10;
                main_tank = 0;
            }
        }

        result
    }
}
