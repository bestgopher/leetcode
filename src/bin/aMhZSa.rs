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
    /// 找到下半部分，翻转上半部分，再比较上下两部分
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut len = 0;
        let mut current = head.as_deref();

        while current.is_some() {
            len += 1;
            current = current.unwrap().next.as_deref();
        }

        let mut dummy = Box::new(ListNode::new(-1));

        for i in 0..len / 2 {
            let next = head.as_deref_mut().unwrap().next.take();
            let n = dummy.next.take();
            head.as_deref_mut().unwrap().next = n;
            dummy.next = head;
            head = next;
        }

        let (mut left, mut right) = if len % 2 == 0 {
            (dummy.next.take(), head)
        } else {
            (dummy.next.take(), head.unwrap().next.take())
        };

        loop {
            match (left, right) {
                (Some(mut x), Some(mut y)) if x.val == y.val => {
                    left = x.next.take();
                    right = y.next.take();
                }
                (None, None) => break,
                _ => return false,
            }
        }

        true
    }
}
