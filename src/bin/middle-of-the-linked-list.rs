#![allow(dead_code, unused, unused_variables, non_snake_case)]

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut s1, mut s2) = (&head, &head);

        while s2.as_ref().unwrap().next.is_some() {
            s1 = &s1.as_ref().unwrap().next;
            s2 = &s2.as_ref().unwrap().next;
            if s2.as_ref().unwrap().next.is_some() {
                s2 = &s2.as_ref().unwrap().next;
            }
        }

        s1.clone()
    }
}
