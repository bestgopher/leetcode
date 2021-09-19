use std::borrow::Borrow;

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let (mut v1, mut v2) = (vec![], vec![]);

        while l1.is_some() || l2.is_some() {
            if let Some(mut x) = l1 {
                v1.push(x.val);
                l1 = x.next.take();
            }

            if let Some(mut x) = l2 {
                v2.push(x.val);
                l2 = x.next.take();
            }
        }

        let mut i = 0; // 进制
        let mut new_node = None;
        loop {
            match (v1.pop(), v2.pop()) {
                (Some(a), Some(b)) => {
                    let mut n = ListNode::new((a + b + i) % 10);
                    n.next = new_node.take();
                    new_node = Some(Box::new(n));
                    i = (a + b + i) / 10;
                }
                (Some(a), None) => {
                    let mut n = ListNode::new((a + i) % 10);
                    n.next = new_node.take();
                    new_node = Some(Box::new(n));
                    i = (a + i) / 10;
                }
                (None, Some(b)) => {
                    let mut n = ListNode::new((b + i) % 10);
                    n.next = new_node.take();
                    new_node = Some(Box::new(n));
                    i = (b + i) / 10;
                }

                (None, None) => {
                    if i > 0 {
                        let mut n = ListNode::new(i);
                        n.next = new_node.take();
                        new_node = Some(Box::new(n));
                    }
                    break;
                }
            }
        }

        new_node
    }
}
