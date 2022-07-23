#![allow(dead_code, unused, unused_variables)]

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
    pub fn merge_k_lists1(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let (mut index, mut min_value) = (0usize, None);
        for (i, v) in lists.iter().enumerate() {
            match v {
                Some(x) => {
                    if min_value.is_none() {
                        min_value = Some(x.val);
                        index = i;
                    } else {
                        if min_value.as_ref().unwrap().gt(&x.val) {
                            min_value = Some(x.val);
                            index = i;
                        }
                    }
                }
                None => (),
            }
        }

        if index == 0 && min_value.is_none() {
            return None;
        }
        let mut root = lists[index].take();
        lists[index] = root.as_mut().unwrap().next.take();

        root.as_mut().unwrap().next = Self::merge_k_lists(lists);

        root
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;

        if lists.is_empty() {
            return None;
        } else if lists.len() == 1 {
            return lists[0].take();
        }

        let mut start = lists[0].take();
        let mut i = 1usize;
        while i < lists.len() {
            start = Self::f(vec![start, lists[i].take()]);
            i += 1;
        }

        start
    }

    fn f(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        match (lists[0].take(), lists[1].take()) {
            (Some(mut x), Some(mut y)) => {
                if x.val < y.val {
                    x.next = Self::f(vec![x.next.take(), Some(y)]);
                    Some(x)
                } else {
                    y.next = Self::f(vec![Some(x), y.next.take()]);
                    Some(y)
                }
            }
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}
