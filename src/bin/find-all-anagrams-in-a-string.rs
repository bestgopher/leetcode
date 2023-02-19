#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 滑动窗口
    /// 记录滑动窗口里面的元素个数和p的元素个数是否相同，相同则表示满足条件
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return Vec::new();
        }

        let mut ans = vec![];
        let mut p_count = [0; 26];

        for &i in p.as_bytes() {
            p_count[(i - b'a') as usize] += 1;
        }

        let mut s_count = [0; 26];

        let bytes = s.as_bytes();

        for i in 0..s.len() {
            if i < p.len() {
                s_count[(bytes[i] - b'a') as usize] += 1;
            } else {
                s_count[(bytes[i] - b'a') as usize] += 1;
                s_count[(bytes[i - p.len()] - b'a') as usize] -= 1;
            }

            if s_count == p_count {
                ans.push((i - p.len() + 1) as i32);
            }
        }

        ans
    }
}
