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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut pre_sum = std::collections::HashMap::new();
        pre_sum.insert(0, dummy.as_ref());
        let mut sum = 0;
        let mut p = dummy.next.as_ref();

        while let Some(n) = p {
            sum += n.val;
            p = n.next.as_ref();
            pre_sum.insert(sum, n.as_ref());
        }

        let mut ans = Box::new(ListNode::new(0));
        let mut p = Some(&mut ans);
        let mut sum = 0;

        while let Some(n) = p {
            sum += n.val;

            if let Some(q) = pre_sum.get(&sum) {
                n.next = match q.next.as_ref() {
                    Some(next) => Some(Box::new(ListNode::new(next.val))),
                    None => None,
                }
            }

            p = n.next.as_mut();
        }

        ans.next
    }
}
