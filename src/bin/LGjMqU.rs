#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

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
    /// 1.双指针找到链表的中心
    /// 2.然后翻转后半部分的链表
    /// 3.合并前后部分
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // 注意：找到后半部分可以使用快慢指针，但是rust我不会
        let mut len = 0;
        let mut current = head.as_deref();
        while current.is_some() {
            len += 1;
            current = current.unwrap().next.as_deref();
        }
        let mut middle = head.as_deref_mut();
        for i in 0..len / 2 {
            middle = middle.unwrap().next.as_deref_mut();
        }

        if middle.is_none() {
            return;
        }

        let mut middle = middle.unwrap().next.take();
        // 翻转middle
        let mut dummy = Box::new(ListNode::new(-1));
        while middle.is_some() {
            let n = dummy.next.take();
            let c = middle.as_deref_mut().unwrap().next.take();

            dummy.next = middle;
            dummy.next.as_deref_mut().unwrap().next = n;

            middle = c;
        }

        let (mut l1, mut l2) = (head.as_deref_mut().unwrap().next.take(), dummy.next.take());

        let mut dummy = Box::new(ListNode::new(-1));
        let mut current = &mut dummy.next;

        loop {
            match (l1, l2) {
                (Some(mut x1), Some(mut x2)) => {
                    l1 = x1.next.take();
                    l2 = x2.next.take();

                    current.insert(x2);
                    current = &mut current.as_deref_mut().unwrap().next;

                    current.insert(x1);
                    current = &mut current.as_deref_mut().unwrap().next;
                }

                (Some(mut x1), None) => {
                    l1 = x1.next.take();
                    l2 = None;

                    current.insert(x1);
                    current = &mut current.as_deref_mut().unwrap().next;
                }

                (None, Some(mut x2)) => {
                    l1 = None;
                    l2 = x2.next.take();

                    current.insert(x2);
                    current = &mut current.as_deref_mut().unwrap().next;
                }

                _ => break,
            }
        }

        head.as_deref_mut().unwrap().next = dummy.next.take();
    }
}
