#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(
        7,
        Solution::max_vowels("ibpbhixfiouhdljnjfflpapptrxgcomvnb".to_string(), 33)
    );
}

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let (mut slow, mut fast) = (0usize, 0usize);
        let s = s.as_bytes();

        if s.len() < k as usize {
            return 0;
        }

        let (mut count, mut max) = (0, 0);
        while fast < s.len() {
            if fast - slow > (k - 1) as usize {
                slow += 1;
                match s[slow - 1] {
                    b'a' | b'e' | b'i' | b'o' | b'u' => count -= 1,
                    _ => (),
                }
            }

            match s[fast] {
                b'a' | b'e' | b'i' | b'o' | b'u' => count += 1,
                _ => (),
            }

            if count > max {
                max = count;
            }

            fast += 1;
        }

        max
    }
}
