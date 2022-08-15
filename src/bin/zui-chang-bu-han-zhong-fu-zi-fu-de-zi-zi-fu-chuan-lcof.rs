#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".into()));
    assert_eq!(1, Solution::length_of_longest_substring("bbbbb".into()));
    assert_eq!(3, Solution::length_of_longest_substring("pwwkew".into()));
    assert_eq!(2, Solution::length_of_longest_substring("au".into()));
    assert_eq!(2, Solution::length_of_longest_substring("aab".into()));
    assert_eq!(3, Solution::length_of_longest_substring("dvdf".into()));
    assert_eq!(5, Solution::length_of_longest_substring("tmmzuxt".into()));
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let (mut start, mut length, mut result) = (0, 0, 0);

        for (index, value) in s.as_bytes().into_iter().enumerate() {
            if let Some(i) = hash.get_mut(value) {
                if start < *i {
                    start = *i;
                }

                length = index - start;
                *i = index;
            } else {
                hash.insert(value, index);
                length += 1;
            }

            result = result.max(length);
        }

        result as i32
    }
}
