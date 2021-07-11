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
        Self::re(l1, l2, 0)
    }

    /// base: 进制，相加和大于10进制
    fn re(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        base: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            if base == 0 {
                return None;
            } else {
                return Some(Box::new(ListNode::new(base)));
            }
        }

        let (mut result, mut value) = (ListNode::new(0), None);
        if l1.is_none() {
            let mut l2 = l2.unwrap();
            result.val = (l2.val + base) % 10;
            let base = (l2.val + base) / 10;
            value = Self::re(None, l2.next.take(), base);
        } else if l2.is_none() {
            let mut l1 = l1.unwrap();
            result.val = (l1.val + base) % 10;
            let base = (l1.val + base) / 10;
            value = Self::re(l1.next.take(), None, base);
        } else {
            let mut l1 = l1.unwrap();
            let mut l2 = l2.unwrap();
            result.val = (l1.val + l2.val + base) % 10;
            let base = (l1.val + l2.val + base) / 10;
            value = Self::re(l1.next.take(), l2.next.take(), base);
        }

        if value.is_some() {
            result.next = value;
        }
        Some(Box::new(result))
    }
}
