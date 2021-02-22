fn main() {}

struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (mut score, mut left, mut right) = (0, 0, 0);
        let s = s.as_bytes();

        for i in 1..s.len() {
            if s[i] == b'1' {
                right += 1;
            }
        }

        for i in 0..s.len() - 1 {
            if s[i] == b'0' {
                left += 1;
            } else if i != 0 && s[i] == b'1' {
                right -= 1;
            }
            if left + right > score {
                score = left + right;
            }
        }

        score
    }
}
