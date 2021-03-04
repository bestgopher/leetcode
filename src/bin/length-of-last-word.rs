fn main() {}

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        let s = s.as_bytes();
        for v in (0..s.len()).rev() {
            if s[v] != b' ' {
                count += 1;
            }

            if count != 0 && s[v] == b' ' {
                break;
            }
        }

        count
    }
}
