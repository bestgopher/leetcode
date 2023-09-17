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
    pub fn remove_duplicate_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut set: std::collections::HashSet<i32> = Default::default();
        let mut tmp = Some(Box::new(ListNode { next: None, val: 0 }));
        let mut current = &mut tmp.as_mut().unwrap().next;
        while head.is_some() {
            let mut h = head.unwrap();
            let next = h.next.take();
            let v = h.val;
            if !set.contains(&v) {
                current.insert(h);
                current = &mut current.as_mut().unwrap().next;
                set.insert(v);
            }

            head = next;
        }

        tmp.unwrap().next.take()
    }
}
