fn main() {}

struct Solution;

impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        let s = num.to_string();
        let s = s.as_bytes();
        let mut count = 0;
        Self::dp(s, &mut count);
        count
    }

    fn dp(s: &[u8], count: &mut i32) {
        if s.len() == 0 {
            *count += 1;
            return;
        }

        if s.len() >= 2 {
            if s[0] == b'1' || (s[0] == b'2' && s[1] <= b'5') {
                Self::dp(&s[2..], count);
            }
        }
        Self::dp(&s[1..], count);
    }
}
