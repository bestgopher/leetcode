#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 双端队列实现
    pub fn final_string(s: String) -> String {
        use std::collections::LinkedList;
        let mut list = LinkedList::new();
        let mut f = true; // true代表正向，false代表逆向

        for i in s.chars() {
            if i == 'i' {
                f = !f;
                continue;
            }

            if f {
                list.push_back(i);
            } else {
                list.push_front(i);
            }
        }

        if f {
            list.into_iter().collect()
        } else {
            list.into_iter().rev().collect()
        }
    }
}
