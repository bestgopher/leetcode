#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let mut d = date.split('-');
        let year = d.next().unwrap().parse::<i32>().unwrap();
        let month = d.next().unwrap().parse::<i32>().unwrap();
        let day = d.next().unwrap().parse::<i32>().unwrap();

        let mut result = 0;
        const MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        for i in 1..month {
            result += MONTH[i as usize - 1];
            if i == 2 && year % 4 == 0 {
                if year % 100 == 0 && year % 400 != 0 {
                    continue;
                }
                result += 1;
            }
        }

        result + day
    }
}
