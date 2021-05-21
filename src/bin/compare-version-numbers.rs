fn main() {}

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (mut s1, mut s2) = (0, 0);
        let (v1, v2) = (version1.as_bytes(), version2.as_bytes());

        loop {
            let (mut r1, mut r2) = (0, 0);
            for i in s1..v1.len() {
                s1 = i + 1;
                if v1[i] == b'.' {
                    break;
                } else {
                    r1 = r1 * 10 + v1[i] - b'0';
                }
            }

            for i in s2..v2.len() {
                s2 = i + 1;
                if v2[i] == b'.' {
                    break;
                } else {
                    r2 = r2 * 10 + v2[i] - b'0';
                }
            }

            if r1 > r2 {
                return 1;
            } else if r1 < r2 {
                return -1;
            }

            if s1 == v1.len() && s2 == v2.len() {
                return 0;
            }
        }
    }
}
