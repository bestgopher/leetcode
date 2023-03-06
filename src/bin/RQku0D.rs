#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert!(Solution::valid_palindrome("aba".into()));
    assert!(Solution::valid_palindrome("abca".into()));
    assert!(!Solution::valid_palindrome("abc".into()));
    assert!(Solution::valid_palindrome("cbbcc".into()));
    assert!(Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".into()));
}

struct Solution;

impl Solution {
    /// 定义两个指针 p1, p2，如果p1 != p2, 则可以移除p1或者p2, 这时只要p1..p2-1 或者p1+1..p2这两个子串是回文串即可以满足删除一行是回文串的需求。
    pub fn valid_palindrome(s: String) -> bool {
        Self::valid(s.as_bytes(), false)
    }

    fn valid(s: &[u8], flag: bool) -> bool {
        if s.is_empty() {
            return true;
        }
        let (mut start, mut end) = (0, s.len() - 1);
        while start < end {
            if s[start] == s[end] {
                start += 1;
                end -= 1;
            } else {
                if !flag {
                    return Self::valid(&s[start + 1..=end], true)
                        || Self::valid(&s[start..=end - 1], true);
                }

                return false;
            }
        }

        true
    }
}
