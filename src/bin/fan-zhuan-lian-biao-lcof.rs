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
    pub fn reverse_list1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = vec![];
        let mut head = head;

        while head.is_some() {
            let mut h = head.unwrap();
            vals.push(h.val);
            head = h.next.take();
        }

        let mut r = None;
        let mut current = &mut r;

        while let Some(val) = vals.pop() {
            current.replace(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }

        r
    }

    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut next = head.as_mut().unwrap().next.take();

        while let Some(n) = next.as_mut() {
            let new_head = n.next.take();
            n.next = head;
            head = next;
            next = new_head;
        }

        head
    }
}
