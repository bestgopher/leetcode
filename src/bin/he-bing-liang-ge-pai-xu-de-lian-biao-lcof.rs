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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut r = None;
        let mut current = &mut r;
        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap().val;
            let v2 = l2.as_ref().unwrap().val;

            if v1 <= v2 {
                current.replace(Box::new(ListNode::new(v1)));
                l1 = l1.unwrap().next.take();
            } else {
                current.replace(Box::new(ListNode::new(v2)));
                l2 = l2.unwrap().next.take();
            }

            current = &mut current.as_mut().unwrap().next;
        }

        if l1.is_some() {
            current.replace(l1.unwrap());
        }

        if l2.is_some() {
            current.replace(l2.unwrap());
        }

        r
    }
}
