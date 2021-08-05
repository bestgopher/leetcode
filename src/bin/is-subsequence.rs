fn main() {}

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut i, mut j) = (0, 0); // i: s'index, j: t'index
        let (s, t) = (s.as_bytes(), t.as_bytes());
        loop {
            if i == s.len() {
                return true;
            }
            if j == t.len() {
                break;
            }

            if s[i] == t[j] {
                i += 1;
            }
            j += 1;
        }

        return false;
    }
}
