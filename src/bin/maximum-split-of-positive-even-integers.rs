#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        let mut result = vec![];
        if final_sum % 2 == 1 {
            return result;
        }

        result.push(2);

        loop {
            final_sum -= result[result.len() - 1];
            if final_sum <= result[result.len() - 1] {
                let last = result[result.len() - 1];
                let len = result.len() - 1;
                result[len] = last + final_sum;
                break;
            } else {
                result.push(result[result.len() - 1] + 2);
            }
        }

        result
    }
}
