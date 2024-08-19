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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut current = &mut result;
        let mut s = 0; // 进制
        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() {
            let a = if let Some(mut x) = l1 {
                l1 = x.next.take();
                x.val
            } else {
                0
            };

            let b = if let Some(mut x) = l2 {
                l2 = x.next.take();
                x.val
            } else {
                0
            };
            let s1 = (a + b + s) / 10;
            let v = (a + b + s) % 10;
            s = s1;
            current = current.next.insert(Box::new(ListNode::new(v)));
        }

        result.next.take()
    }
}
