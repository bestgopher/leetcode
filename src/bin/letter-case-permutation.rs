fn main() {}

struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut v = Vec::new();
        Self::func(s, &mut v, 0);
        v
    }

    fn func(s: String, v: &mut Vec<String>, index: usize) {
        let mut new_s = s.clone();
        let mut new_s = unsafe { new_s.as_bytes_mut() };
        v.push(s);
        if index == new_s.len() {
            return;
        }
        let mut new_index = index;
        for i in index..new_s.len() {
            new_index = i;
            if new_s[i] >= b'a' && new_s[i] <= b'z' {
                new_s[i] -= 32;
                Self::func(
                    unsafe { String::from_utf8_unchecked(new_s.to_vec()) },
                    v,
                    new_index + 1,
                );
                new_s[i] += 32;
            } else if new_s[i] >= b'A' && new_s[i] <= b'Z' {
                new_s[i] += 32;
                Self::func(
                    unsafe { String::from_utf8_unchecked(new_s.to_vec()) },
                    v,
                    new_index + 1,
                );
                new_s[i] -= 32;
            }
        }
    }
}
