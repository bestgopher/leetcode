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
        let mut a = vec![];
        let mut current = l1.as_deref();
        while current.is_some() {
            let c = current.unwrap();
            a.push(c.val);
            current = c.next.as_deref();
        }

        let mut b = vec![];
        let mut current = l2.as_deref();
        while current.is_some() {
            let c = current.unwrap();
            b.push(c.val);
            current = c.next.as_deref();
        }

        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut f = 0;
        loop {
            let val = match (a.pop(), b.pop()) {
                (Some(x), Some(y)) => {
                    let mut v = x + y + f;
                    f = 0;
                    if v > 9 {
                        f = v / 10;
                        v = v % 10;
                    }

                    v
                }

                (Some(x), None) => {
                    let mut v = x + f;
                    f = 0;
                    if v > 9 {
                        f = v / 10;
                        v = v % 10;
                    }

                    v
                }

                (None, Some(y)) => {
                    let mut v = y + f;
                    f = 0;
                    if v > 9 {
                        f = v / 10;
                        v = v % 10;
                    }

                    v
                }

                (None, None) => break,
            };

            let n = dummy.as_deref_mut().and_then(|x| x.next.take());
            let mut c = Box::new(ListNode { val, next: n });

            dummy.as_deref_mut().unwrap().next.insert(c);
        }

        if f != 0 {
            let n = dummy.as_deref_mut().and_then(|x| x.next.take());
            let mut c = Box::new(ListNode { val: f, next: n });

            dummy.as_deref_mut().unwrap().next.insert(c);
        }

        dummy.unwrap().next
    }
}
