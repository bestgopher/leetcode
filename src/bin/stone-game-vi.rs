#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut x: Vec<_> = alice_values
            .into_iter()
            .zip(bob_values.into_iter())
            .map(|(x, y)| (x + y, x, y))
            .collect();

        x.sort_unstable_by(|x, y| x.cmp(y).reverse());

        let (mut a_score, mut b_score) = (0, 0);
        for i in 0..x.len() {
            if i % 2 == 0 {
                a_score += x[i].1;
            } else {
                b_score += x[i].2;
            }
        }

        match a_score.cmp(&b_score) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
        }
    }
}
