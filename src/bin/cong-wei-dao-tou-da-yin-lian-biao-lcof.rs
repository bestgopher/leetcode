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
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut head = head;

        while head.is_some() {
            let h = head.unwrap();
            result.push(h.val);
            head = h.next;
        }

        result.reverse();

        result
    }
}
