#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut stack = std::collections::VecDeque::new();
        let mut next = head.as_mut().unwrap().next.take();
        while next.is_some() {
            let new = next.as_mut().unwrap().next.take();
            stack.push_back(next);
            next = new;
        }
        let mut flag = false; // false标识从栈后面拿
        while !stack.is_empty() {
            let next = if flag {
                stack.pop_front().unwrap()
            } else {
                stack.pop_back().unwrap()
            };

            head.as_mut().unwrap().next = next;
            head = &mut head.as_mut().unwrap().next;

            flag = !flag;
        }
    }
}
