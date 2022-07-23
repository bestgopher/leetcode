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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut x), Some(mut y)) => {
                if x.val <= y.val {
                    x.next = Self::merge_two_lists(x.next, Some(y));
                    Some(x)
                } else {
                    y.next = Self::merge_two_lists(Some(x), y.next);
                    Some(y)
                }
            }
            (None, Some(y)) => Some(y),
            (Some(x), None) => Some(x),
            (None, None) => None,
        }
    }
}
