fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut word = word;
        let mut i = 0;
        for (index, value) in word.chars().enumerate() {
            if value == ch {
                i = index;
            }
        }

        unsafe {
            let mut s = word.as_bytes_mut();
            let mut j = 0;
            while j < i {
                s.swap(i, j);
                i -= 1;
                j += 1;
            }
        }

        word
    }
}
