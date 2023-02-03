#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::distinct_subseq_ii("abc".into()), 7);
    assert_eq!(Solution::distinct_subseq_ii("aaa".into()), 3);
}

struct Solution;

impl Solution {
    /// 暴力解法
    /// 从长度为1开始，
    /// 从下标为0开始
    ///
    /// dp
    /// v[i]表示s[0..i]的子串个数
    /// 则v[i] = v[i-1] + 新增个数a - 重复个数b
    /// 因为s[0..i]相较于s[0..i-1]新增了一个字符，所以 a = v[i-1]+1
    /// 新增的子串都是以s[i]结尾的，所以重复的肯定是上次遇到相同字符结尾的字符串子串的个数，因此记录下每个字符上次出现的个数，即为重复的个数
    pub fn distinct_subseq_ii(s: String) -> i32 {
        // let mut v = vec![0; s.len()]; // 这种只需要前一个数据，不需要回溯的，用一个变量表示就行了
        let mut repeate = [0; 26];
        let bytes = s.as_bytes();

        let mut v = 1;
        repeate[(bytes[0] - b'a') as usize] = v;

        for i in 1..bytes.len() {
            v = (v * 2 + 1) - repeate[(bytes[i] - b'a') as usize];
            repeate[(bytes[i] - b'a') as usize] = v;
        }

        v
    }
}
