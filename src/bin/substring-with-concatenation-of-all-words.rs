#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    assert_eq!(
        Solution::find_substring("aaaaaa".into(), vec!["a".into(), "a".into()]),
        vec![0, 1, 2, 3, 4]
    );

    assert_eq!(
        Solution::find_substring(
            "barfoothefoobarman".into(),
            vec!["foo".into(), "bar".into()]
        ),
        vec![0, 9]
    );

    assert_eq!(
        Solution::find_substring(
            "wordgoodgoodgoodbestword".into(),
            vec!["word".into(), "good".into(), "best".into(), "word".into()]
        ),
        vec![]
    );

    assert_eq!(
        Solution::find_substring(
            "barfoofoobarthefoobarman".into(),
            vec!["bar".into(), "foo".into(), "the".into()]
        ),
        vec![6, 9, 12]
    );
}

struct Solution;

impl Solution {
    /// 先对words的单词计数，使用hashmap，m
    /// 遍历s的下标为i，获取words单词长度的子字符串，如果在m存在，如果数量大于1则数量-1，如果数量为1则从m中删除此字符串，如果不存在则说明i为起始的子字符串不满足条件
    /// 注意：每次遍历时都要生成一个新的m
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let sub_string_len = words.len() * word_len;
        let mut ans = vec![];

        if s.len() < sub_string_len {
            return ans;
        }

        let mut map = std::collections::HashMap::new();

        for i in words.iter() {
            map.entry(&i[..]).and_modify(|x| *x += 1).or_insert(1i32);
        }

        'Loop: for i in 0..=s.len() - sub_string_len {
            let mut m = map.clone();

            for j in (i..i + sub_string_len).step_by(word_len) {
                match m.get_mut(&s[j..j + word_len]) {
                    Some(x) => {
                        if *x > 1 {
                            *x -= 1
                        } else if *x == 1 {
                            m.remove(&s[j..j + word_len]);
                        }
                    }
                    None => continue 'Loop,
                }
            }

            if m.is_empty() {
                ans.push(i as i32);
            }
        }

        ans
    }
}
