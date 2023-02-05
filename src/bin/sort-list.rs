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
    /// 归并排序
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut ptr = head.as_ref();

        if ptr.is_some() {
            ptr = ptr.unwrap().next.as_ref();
            length += 1;
        }

        Solution::sort(head, length)
    }

    pub fn sort(mut head: Option<Box<ListNode>>, length: usize) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        if head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut middle = head.as_mut();

        for _ in 0..length / 2 {
            middle = middle.unwrap().next.as_mut();
        }

        let mut r1 = Solution::sort_list(middle.unwrap().next.take());
        let mut r2 = Solution::sort_list(head);

        let mut result = Some(Box::new(ListNode::new(-1i32)));
        let mut current = result.as_mut();
        loop {
            match (r1, r2) {
                (Some(mut x1), Some(mut x2)) => {
                    if x1.val < x2.val {
                        r2 = Some(x2);
                        r1 = x1.next.take();
                        current.as_mut().unwrap().next = Some(x1);
                    } else {
                        r1 = Some(x1);
                        r2 = x2.next.take();
                        current.as_mut().unwrap().next = Some(x2);
                    }

                    current = current.unwrap().next.as_mut();
                }
                (Some(x1), None) => {
                    current.unwrap().next = Some(x1);
                    break;
                }
                (None, Some(x2)) => {
                    current.unwrap().next = Some(x2);
                    break;
                }
                _ => break,
            }
        }

        result.map(|mut x| x.next.take()).unwrap()
    }
}
