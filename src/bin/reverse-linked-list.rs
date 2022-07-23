#![allow(dead_code, unused, unused_variables)]

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut v = vec![];
        let mut head = head;

        while head.is_some() {
            let h = head.as_mut().unwrap().next.take();
            v.push(head);
            head = h;
        }

        let mut root = v.pop().unwrap();
        let mut s = &mut root;
        while !v.is_empty() {
            let node = v.pop().unwrap();
            s.as_mut().unwrap().next = node;
            s = &mut s.as_mut().unwrap().next;
        }

        root
    }
}
