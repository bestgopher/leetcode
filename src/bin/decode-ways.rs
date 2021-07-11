fn main() {
    println!("{}", Solution::num_decodings("12".to_string()));
    println!("{}", Solution::num_decodings("06".to_string()));
    println!(
        "{}",
        Solution::num_decodings("2611055971756562".to_string())
    );
    println!(
        "{}",
        Solution::num_decodings("111111111111111111111111111111111111111111111".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.starts_with('0') {
            return 0;
        }

        let mut v = vec![-1; s.len()];

        Self::f(s.as_bytes(), &mut v)
    }

    fn f(s: &[u8], v: &mut Vec<i32>) -> i32 {
        let mut r = 0;
        let l = s.len();

        if l > 0 {
            if v[l - 1] != -1 {
                r = v[l - 1];
            } else {
                if s[0] != b'0' {
                    if l > 1 {
                        if (s[0] == b'2' && s[1] <= b'6') || s[0] < b'2' {
                            r += Self::f(&s[2..], v);
                        }

                        if s[1] != b'0' {
                            r += Self::f(&s[1..], v);
                        }
                    } else {
                        r += Self::f(&s[1..], v);
                    }
                }
            }
            v[l - 1] = r;
        } else {
            r = 1;
        }
        r
    }
}
