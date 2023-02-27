#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    assert_eq!(
        Solution::max_product(
            vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        ),
        16
    )
}

struct Solution;

impl Solution {
    /// 用一个i32表示 b'a' - b'z' 存在的数，比如存在b'a'，则此i32的最低位为1
    /// 然后两个数相与的结果为0的话则表示不存在相同的字母。
    /// 然后长度相乘得到结果。
    pub fn max_product(words: Vec<String>) -> i32 {
        let v = words
            .iter()
            .map(|s| {
                let mut i = 0;
                for x in s.as_bytes() {
                    i |= (1 << (x - b'a'));
                }
                i
            })
            .collect::<Vec<_>>();

        let mut ans = 0;

        for i in 0..v.len() {
            for j in 0..v.len() {
                if i == j {
                    continue;
                }

                if v[i] & v[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }

        ans as i32
    }
}
