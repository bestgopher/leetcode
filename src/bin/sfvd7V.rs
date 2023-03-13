#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::<[u8; 26], Vec<String>>::new();

        for s in strs {
            let mut v = [0u8; 26];
            for &j in s.as_bytes() {
                v[(j - b'a') as usize] += 1;
            }

            match map.entry(v) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(s);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(vec![s]);
                }
            }
        }

        map.into_iter().map(|(x, y)| y).collect()
    }
}
