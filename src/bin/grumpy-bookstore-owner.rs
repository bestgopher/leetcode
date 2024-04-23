#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut sum = 0;
        let mut customers = customers;

        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                sum += customers[i];
                customers[i] = 0;
            }

            if i > 0 {
                customers[i] += customers[i - 1];
            }
        }

        let mut x = customers[minutes as usize - 1];
        for i in minutes as usize..customers.len() {
            x = x.max(customers[i] - customers[i - minutes as usize]);
        }

        sum + x
    }
}
