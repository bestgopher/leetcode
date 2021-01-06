fn main() {}

struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        if s == "".to_string() {
            return 0;
        }

        let mut num = 0;
        let mut flag = true;

        for i in s.chars().into_iter() {
            if flag && i != ' ' {
                num += 1;
                flag = false;
            }

            if i == ' ' {
                flag = true;
            }
        }

        num
    }
}
