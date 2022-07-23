#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(2, Solution::length_of_longest_substring("aab".to_string()));
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    assert_eq!(2, Solution::length_of_longest_substring("au".to_string()));
    assert_eq!(
        5,
        Solution::length_of_longest_substring("tmmzuxt".to_string())
    );
    assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
    assert_eq!(3, Solution::length_of_longest_substring("dvdf".to_string()));
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }

        let (mut max, mut slow) = (0, 0);
        let mut hash = std::collections::HashMap::new();

        for (index, value) in s.as_bytes().into_iter().enumerate() {
            if let Some(&v) = hash.get(value) {
                if v >= slow {
                    slow = v + 1;
                }
            }

            hash.insert(*value, index);
            max = max.max(index - slow + 1);
        }

        max as i32
    }
}
