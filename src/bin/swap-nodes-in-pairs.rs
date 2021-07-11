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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut x) => match x.next.take() {
                Some(mut y) => {
                    x.next = Self::swap_pairs(y.next.take());
                    y.next = Some(x);
                    Some(y)
                }
                None => Some(x),
            },
            None => None,
        }
    }
}
