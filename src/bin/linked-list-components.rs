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
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<std::collections::HashSet<_>>();
        let mut head = head;
        let mut result = 0;
        let mut flag = false;
        while head.is_some() {
            let v = head.as_ref().unwrap().val;
            if set.contains(&v) {
                if !flag {
                    result += 1;
                    flag = true;
                }
            } else {
                flag = false;
            }

            head = head.as_mut().unwrap().next.take();
        }

        result
    }
}
