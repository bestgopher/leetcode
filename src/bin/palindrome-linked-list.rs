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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut v = vec![];
        let mut head = head;

        while head.is_some() {
            v.push(head.as_ref().unwrap().val);
            head = head.as_mut().unwrap().next.take();
        }

        let (mut star, mut end) = (0, v.len() - 1);
        while star < end {
            if v[star] != v[end] {
                return false;
            }
            star += 1;
            end -= 1;
        }

        true
    }
}
