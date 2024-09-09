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
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut next = &mut result.next;
        let mut current = head.unwrap().next;

        let mut val = 0;
        while current.is_some() {
            if current.as_ref().unwrap().val == 0 {
                next.insert(Box::new(ListNode::new(val)));
                val = 0;
                next = &mut next.as_mut().unwrap().next;
            } else {
                val += current.as_ref().unwrap().val;
            }

            current = current.unwrap().next;
        }

        result.next.take()
    }
}
