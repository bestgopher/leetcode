#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        let mut hash = std::collections::HashMap::new();
        Self::f(&satisfaction[..], &mut hash, 0, 1)
    }

    fn f(
        satisfaction: &[i32],
        hash: &mut std::collections::HashMap<(usize, i32), i32>,
        start_index: usize,
        current_time: i32,
    ) -> i32 {
        if start_index == satisfaction.len() {
            return 0;
        }

        let x = if let Some(x) = hash.get(&(start_index + 1, current_time + 1)) {
            *x
        } else {
            let v = Self::f(satisfaction, hash, start_index + 1, current_time + 1);
            hash.insert((start_index + 1, current_time + 1), v);
            v
        };

        let x = x + satisfaction[start_index] * current_time;

        let y = if let Some(x) = hash.get(&(start_index + 1, current_time)) {
            *x
        } else {
            let v = Self::f(satisfaction, hash, start_index + 1, current_time);
            hash.insert((start_index + 1, current_time), v);
            v
        };

        let max = x.max(y);
        hash.insert((start_index, current_time), max);

        max
    }
}
