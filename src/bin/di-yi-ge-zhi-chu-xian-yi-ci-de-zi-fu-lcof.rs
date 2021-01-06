fn main() {}

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        if s == "".to_string() {
            return ' ';
        }
        let mut m = std::collections::HashMap::new();
        for &i in s.as_bytes() {
            m.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        for &i in s.as_bytes() {
            if let Some(s) = m.get(&i) {
                if *s == 1 {
                    return i as char;
                }
            }
        }

        ' '
    }
}
