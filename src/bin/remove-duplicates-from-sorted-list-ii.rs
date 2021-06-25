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
        let s = Self::func(head);
        s.0
    }

    fn func(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<i32>) {
        if head.is_none() {
            return (None, None);
        }
        let v = head.as_ref().unwrap().val;
        let (node, val) = Self::func(head.as_mut().unwrap().next.take());
        if val.is_none() {
            (head, Some(v))
        } else {
            if val.unwrap() == v {
                if node.is_none() {
                    (None, Some(v))
                } else {
                    if node.as_ref().unwrap().val == v {
                        (node.unwrap().next, Some(v))
                    } else {
                        (node, Some(v))
                    }
                }
            } else {
                head.as_mut().unwrap().next = node;
                (head, Some(v))
            }
        }
    }
}
