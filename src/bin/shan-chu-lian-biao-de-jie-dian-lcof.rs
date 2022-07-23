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
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let (mut node, mut last_node) = (head, result.as_mut());
        while node.is_some() {
            let mut n = node.unwrap();
            if n.val != val {
                if let Some(i) = last_node {
                    (*i).next = Some(Box::new(ListNode::new(n.val)));
                    last_node = i.next.as_mut();
                } else {
                    result = Some(Box::new(ListNode::new(n.val)));
                    last_node = result.as_mut();
                }
            }
            node = n.next.take();
        }

        result
    }
}
