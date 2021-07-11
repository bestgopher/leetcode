fn main() {}

struct Solution;

static V: &[&str; 10] = &[
    "", "!@#", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        Self::scan(digits.as_bytes())
    }

    fn scan(s: &[u8]) -> Vec<String> {
        if s.len() == 1 {
            let mut v = vec![];

            for i in V[(s[0] - b'0') as usize].chars() {
                v.push(i.to_string());
            }
            return v;
        }

        let mut v1 = vec![];

        let s1 = Self::scan(&s[1..]);

        for i in V[(s[0] - b'0') as usize].chars() {
            for j in s1.iter() {
                let mut s2 = String::new();
                s2.push(i);
                s2.push_str(j.as_str());
                v1.push(s2);
            }
        }

        v1
    }
}
