fn main() {}

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut flag = false;

        for (i, &v) in word.as_bytes().iter().enumerate() {
            if i == 0 && v >= b'A' && v <= b'Z' {
                flag = true;
            } else {
                if v >= b'A' && v <= b'Z' {
                    if !flag {
                        return false;
                    }
                } else {
                    if flag && i != 1 {
                        return false;
                    }
                    flag = false;
                }
            }
        }

        true
    }
}
