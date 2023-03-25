#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let left = Self::prune_tree(root.clone().unwrap().borrow_mut().left.take());
        let right = Self::prune_tree(root.clone().unwrap().borrow_mut().right.take());

        if left.is_none() && right.is_none() && root.clone().unwrap().borrow().val == 0 {
            return None;
        }

        root.clone().unwrap().borrow_mut().left = left;
        root.clone().unwrap().borrow_mut().right = right;

        root
    }
}
