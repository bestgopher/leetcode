#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!(
        "{:?}",
        Solution::find_repeated_dna_sequences("AAAAAAAAAAA".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut v = std::collections::HashMap::new();
        let mut r = vec![];

        for i in 10..=s.len() {
            // v.entry(&s[i - 10..i]).and_modify(|x| *x += 1).or_insert(1);
            if let Some(x) = v.get_mut(&s[i - 10..i]) {
                if *x == 1 {
                    r.push(s[i - 10..i].to_string());
                }
                *x += 1;
            } else {
                v.insert(&s[i - 10..i], 1);
            }
        }

        r
    }
}
