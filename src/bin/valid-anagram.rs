fn main() {}

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut v = [0; 26];
        for &i in s.as_bytes().iter() {
            v[(i - b'a') as usize] += 1;
        }

        for &i in t.as_bytes().iter() {
            v[(i - b'a') as usize] -= 1;
            if v[(i - b'a') as usize] < 0 {
                return false;
            }
        }

        true
    }
}
