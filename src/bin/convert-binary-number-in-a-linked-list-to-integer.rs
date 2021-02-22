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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut v = vec![];
        let mut root = head;
        while root.is_some() {
            v.push(root.as_ref().unwrap().val);
            root = root.unwrap().next;
        }

        let (mut num, mut a) = (0, 1);
        for i in (0..v.len()).rev().into_iter() {
            num += a * v[i];
            a *= 2;
        }

        num
    }
}
