fn main() {}

struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut l_num = 0;
        let mut r_num = 0;
        let mut num = 0;

        for i in s.chars().into_iter() {
            if i == 'L' {
                l_num += 1;
            }

            if i == 'R' {
                r_num += 1;
            }

            if l_num == r_num {
                num += 1;
                l_num = 0;
                r_num = 0;
            }
        }

        num
    }
}
