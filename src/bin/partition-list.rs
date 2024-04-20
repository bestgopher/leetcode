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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (ListNode::new(-1), ListNode::new(-1));
        let (mut ptr1, mut ptr2) = (&mut l1.next, &mut l2.next);

        let mut head = head;

        while head.is_some() {
            let mut h = head.unwrap();
            head = h.next.take();

            if h.val < x {
                *ptr1 = Some(h);
                ptr1 = ptr1.as_mut().map(|mut x| &mut x.next).unwrap();
            } else {
                *ptr2 = Some(h);
                ptr2 = ptr2.as_mut().map(|mut x| &mut x.next).unwrap();
            }
        }

        *ptr1 = l2.next.take();

        l1.next
    }
}
