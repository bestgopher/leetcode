#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut v1 = vec![];
        let mut v2 = vec![]; // 用于计数的栈
        let mut last = 0;

        for i in s.into_bytes().into_iter() {
            v1.push(i);
            if i != last {
                last = i;
                v2.push(1);
            } else {
                let r = v2.pop().unwrap();
                v2.push(r + 1);
                if let Some(x) = v2.last() {
                    if *x == k {
                        v2.pop();
                        v1 = v1[0..v1.len() - k as usize].to_vec();
                        if v1.is_empty() {
                            last = 0;
                        } else {
                            last = v1[v1.len() - 1];
                        }
                    }
                }
            }
        }

        String::from_utf8(v1).unwrap()
    }
}
