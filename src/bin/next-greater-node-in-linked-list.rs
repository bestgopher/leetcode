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
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        while let Some(l) = head.take() {
            v.push(l.val);
            head = l.next;
        }

        let (mut stack, mut res) = (vec![], vec![0; v.len()]);

        for i in 0..v.len() {
            if stack.len() == 0 {
                stack.push(i);
            } else {
                while let Some(&t) = stack.last() {
                    if v[i] <= v[t] {
                        break;
                    }
                    res[stack.pop().unwrap()] = v[i];
                }

                stack.push(i);
            }
        }

        res
    }
}
