#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        Self::dfs(head.clone(), root.clone())
            || Self::is_sub_path(
                head.clone(),
                root.clone().and_then(|x| x.borrow().left.clone()),
            )
            || Self::is_sub_path(
                head.clone(),
                root.clone().and_then(|x| x.borrow().right.clone()),
            )
    }

    fn dfs(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (head.clone(), root.clone()) {
            (Some(h), Some(r)) => {
                if h.val != r.borrow().val {
                    return false;
                }

                Self::dfs(h.next.clone(), r.borrow().left.clone())
                    || Self::dfs(h.next.clone(), r.borrow().right.clone())
            }
            (Some(h), None) => false,
            _ => true,
        }
    }
}
