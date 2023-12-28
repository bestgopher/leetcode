#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = vec![words[0].clone()];
        for i in 1..words.len() {
            if !Self::f(words[i - 1].as_bytes(), words[i].as_bytes()) {
                result.push(words[i].clone());
            }
        }

        result
    }

    fn f(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut map = std::collections::HashMap::new();
        for &i in a {
            map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        for i in b {
            if map.contains_key(i) {
                if map[i] == 1 {
                    map.remove(i);
                } else {
                    map.entry(*i).and_modify(|x| *x -= 1);
                }
            }
        }

        map.is_empty()
    }
}
