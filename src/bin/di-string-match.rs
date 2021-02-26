fn main() {}

struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut v = Vec::with_capacity(s.len());
        let (mut m, mut n) = (0, s.len() as i32);

        for &i in s.as_bytes().into_iter() {
            if i == b'I' {
                v.push(m);
                m += 1;
            } else {
                v.push(n);
                n -= 1;
            }
        }

        v.push(m);

        v
    }
}
