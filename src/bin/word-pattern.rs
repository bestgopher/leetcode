fn main() {}

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut m = std::collections::HashMap::<u8, &str>::new();
        let mut m1 = std::collections::HashMap::<&str, u8>::new();
        let s = s.split(' ').collect::<Vec<&str>>();
        let pattern = pattern.as_bytes();

        if s.len() != pattern.len() {
            return false;
        }

        for i in 0..s.len() {
            if let Some(&v) = m1.get(&s[i]) {
                if v != pattern[i] {
                    return false;
                }
            }

            if let Some(&v) = m.get(&pattern[i]) {
                if v != s[i] {
                    return false;
                }
            }

            m1.insert(s[i], pattern[i]);
            m.insert(pattern[i], s[i]);
        }

        true
    }
}
