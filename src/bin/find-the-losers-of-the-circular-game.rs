#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut set = std::collections::HashSet::new();
        let mut current = 1;
        for i in 1.. {
            set.insert(current);
            current = if (i * k + current) <= n {
                i * k + current
            } else if (i * k + current) % n == 0 {
                n
            } else {
                (i * k + current) % n
            };

            if set.contains(&current) {
                break;
            }
        }

        (1..=n).filter(|i| !set.contains(i)).collect()
    }
}
