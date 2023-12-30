#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut start = 3i32;

        for i in 1970..year {
            start += 365;
            if i % 4 == 0 {
                if i % 100 == 0 && i % 400 != 0 {
                    continue;
                }

                start += 1;
            }
        }

        const MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for i in 1..month {
            start += MONTH[i as usize - 1];
            if i == 2 && year % 4 == 0 {
                if year % 100 == 0 && year % 400 != 0 {
                    continue;
                }
                start += 1;
            }
        }

        start += day;

        match start % 7 {
            0 => "Sunday".into(),
            1 => "Monday".into(),
            2 => "Tuesday".into(),
            3 => "Wednesday".into(),
            4 => "Thursday".into(),
            5 => "Friday".into(),
            6 => "Saturday".into(),
            _ => unreachable!(),
        }
    }
}
