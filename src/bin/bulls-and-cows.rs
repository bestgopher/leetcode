#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_hint1(secret: String, guess: String) -> String {
        let (secret, guess) = (secret.as_bytes(), guess.as_bytes());
        let mut count = std::collections::HashMap::new();
        for &i in secret {
            count.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let (mut bulls, mut cows_1) = (0, 0);

        for i in 0..secret.len() {
            if secret[i] == guess[i] {
                bulls += 1;
            }

            if let Some(x) = count.get_mut(&guess[i]) {
                if *x > 0 {
                    *x -= 1;
                    cows_1 += 1;
                }
            }
        }

        let cows = cows_1 - bulls;

        format!("{bulls}A{cows}B")
    }

    pub fn get_hint(secret: String, guess: String) -> String {
        let (mut c1, mut c2) = ([0; 10], [0; 10]);
        let mut bulls = 0;
        let (mut secret, mut guess) = (secret.as_bytes(), guess.as_bytes());
        for i in 0..secret.len() {
            if secret[i] == guess[i] {
                bulls += 1;
            } else {
                c1[(secret[i] - b'0') as usize] += 1;
                c2[(guess[i] - b'0') as usize] += 1;
            }
        }

        let cows: i32 = (0..10).map(|x| c1[x].min(c2[x])).sum();

        format!("{bulls}A{cows}B")
    }
}
