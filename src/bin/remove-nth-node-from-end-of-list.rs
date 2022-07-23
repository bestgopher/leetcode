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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (node, _) = Self::func(head, n);
        node
    }

    fn func(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        if head.is_none() {
            return (None, 0);
        }

        let mut head = head;
        let next = head.as_mut().unwrap().next.take();
        let (node, num) = Self::func(next, n);
        if num + 1 == n {
            return (node, num + 1);
        }

        head.as_mut().unwrap().next = node;
        (head, num + 1)
    }
}
