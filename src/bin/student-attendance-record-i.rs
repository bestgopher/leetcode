fn main() {}

struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let (mut a_count, mut l_count) = (0, 0);
        let vec = s.into_bytes();

        for (i, &v) in vec.iter().enumerate() {
            if v == b'A' {
                a_count += 1;
                if a_count > 1 {
                    return false;
                }
            }

            if vec[i] == b'L' {
                l_count += 1;
                if l_count > 2 {
                    return false;
                }
            } else {
                l_count = 0;
            }
        }

        true
    }
}
