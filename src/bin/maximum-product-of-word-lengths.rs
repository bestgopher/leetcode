fn main() {}

struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut v = vec![0i32; words.len()];

        for i in 0..words.len() {
            for &j in words[i].as_bytes() {
                v[i] |= (1 << (j - b'a'));
            }
        }

        let mut r = 0;

        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if (v[i] & v[j]) == 0 {
                    r = r.max(words[i].len() * words[j].len())
                }
            }
        }

        r as i32
    }
}
