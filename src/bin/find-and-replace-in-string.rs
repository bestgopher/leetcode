#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut map1: std::collections::HashMap<_, _> = std::collections::HashMap::new();
        let mut map2: std::collections::HashMap<_, _> = std::collections::HashMap::new();

        for i in 0..indices.len() {
            map1.insert(indices[i] as usize, &sources[i]);
            map2.insert(indices[i] as usize, i);
        }

        let mut result = Vec::new();
        let mut i = 0;

        while i < s.len() {
            if let Some(x) = map1.get(&i) {
                if s[i..].starts_with(x.as_str()) {
                    result.extend_from_slice(targets[*map2.get(&i).unwrap()].as_bytes());
                    i += x.len();
                    continue;
                }
            }

            result.push(s.as_bytes()[i]);
            i += 1;
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}
