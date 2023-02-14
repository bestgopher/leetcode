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

        while ptr.is_some() {
            length += 1;
            ptr = ptr.and_then(|x| x.next.as_ref());
        }

        Self::sort(head, length)
    }

    pub fn sort(mut head: Option<Box<ListNode>>, length: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut middle = head.as_mut();

        for _j in 0..length / 2 - 1 {
            middle = middle.and_then(|x| x.next.as_mut());
        }

        let right = middle.and_then(|x| x.next.take());

        let mut right = Self::sort(right, length - (length / 2));
        let mut left = Self::sort(head, length / 2);

        let mut ans = Some(Box::new(ListNode::new(-1)));
        let mut current = &mut ans.as_mut().unwrap().next;

        while left.is_some() || right.is_some() {
            if left.is_none() {
                current.insert(right.unwrap());
                break;
            } else if right.is_none() {
                current.insert(left.unwrap());
                break;
            } else {
                if left.as_ref().unwrap().val < right.as_ref().unwrap().val {
                    let new_left = left.as_mut().unwrap().next.take();
                    current.insert(left.unwrap());
                    left = new_left;
                } else {
                    let new_right = right.as_mut().unwrap().next.take();
                    current.insert(right.unwrap());
                    right = new_right;
                }

                current = &mut current.as_mut().unwrap().next;
            }
        }

        ans.and_then(|mut x| x.next.take())
    }
}
