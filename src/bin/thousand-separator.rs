fn main() {}

struct Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string();
        if n < 1000 {
            return s;
        }
        let mut l = s.len() / 3;
        if s.len() % 3 != 0 {
            l += 1;
        }
        let mut v = vec![String::new(); l];
        let mut index = s.len();
        while index > 0 {
            if index < 3 {
                v[l - 1] = s[0..index].to_string();
                index = 0;
            } else {
                v[l - 1] = s[index - 3..index].to_string();
                index -= 3;
            }
            l -= 1;
        }

        v.join(".")
    }
}
