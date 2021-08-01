fn main() {}

struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut h = vec![0u8; 26];

        for i in s.bytes() {
            h[(i - b'a') as usize] += 1;
        }

        for i in t.bytes() {
            if h[(i - b'a') as usize] == 0 {
                return i as char;
            }

            h[(i - b'a') as usize] -= 1;
        }

        'a'
    }
}
