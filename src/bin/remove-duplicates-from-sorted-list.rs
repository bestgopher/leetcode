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
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head;
        let node = Self::delete_duplicates(head.as_mut().unwrap().next.take());
        if node.is_some() && head.as_ref().unwrap().val == node.as_ref().unwrap().val {
            node
        } else {
            head.as_mut().unwrap().next = node;
            head
        }
    }
}
