#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::minimize_result("12+34".to_string()),
        "1(2+3)4".to_string()
    );
}

struct Solution;

impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let mut expression = expression.split('+');
        let left = expression.next().unwrap();
        let right = expression.next().unwrap();

        let mut left_index = 0;
        let mut right_index = 0;
        let mut min = i32::MAX;

        for i in 0..left.len() {
            let l1 = if i == 0 {
                1
            } else {
                left[..i].parse::<i32>().unwrap()
            };

            let l2 = if i < left.len() {
                left[i..].parse::<i32>().unwrap()
            } else {
                1
            };

            for j in 1..=right.len() {
                let l3 = if j == 0 {
                    1
                } else {
                    right[..j].parse::<i32>().unwrap()
                };

                let l4 = if j < right.len() {
                    right[j..].parse::<i32>().unwrap()
                } else {
                    1
                };

                let c = l1 * (l2 + l3) * l4;

                if c < min {
                    left_index = i;
                    right_index = j;
                    min = c;
                }
            }
        }

        format!(
            "{}({}+{}){}",
            &left[..left_index],
            &left[left_index..],
            &right[..right_index],
            &right[right_index..]
        )
    }
}
