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
    pub fn get_kth_from_end(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut h = head.as_ref();
        let mut len = 0;

        while h.is_some() {
            len += 1;
            h = h.unwrap().next.as_ref();
        }

        while len > k {
            head = head.unwrap().next.take();
            len -= 1;
        }

        head
    }
}
