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
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut stack = vec![];
        while let Some(mut x) = head.take() {
            while let Some(m) = stack.pop() {
                if m >= x.val {
                    stack.push(m);
                    break;
                }
            }
            stack.push(x.val);
            head = x.next.take();
        }

        let mut pre_head = ListNode { val: 0, next: None };
        let mut current = &mut pre_head.next;

        for x in stack {
            current.insert(Box::new(ListNode { val: x, next: None }));
            current = &mut current.as_mut().unwrap().next;
        }

        pre_head.next.take()
    }
}
