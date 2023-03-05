#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
}

struct Solution;

impl Solution {
    /// 使用set来判断是否已经存在，如果存在的话，移动p直到把重复的元素移除为止
    /// abcabcbb
    /// 如果当前下标为4，p需要移动到2，也就是把重复的b字符移除为止，这时就不包含重复的字符了。
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut ans = 0;
        let mut p = 0;

        for i in 0..s.len() {
            while set.contains(&(s.as_bytes()[i])) {
                set.remove(&(s.as_bytes()[p]));
                p += 1;
            }

            set.insert(s.as_bytes()[i]);
            ans = ans.max(i - p + 1);
        }

        ans as i32
    }
}
