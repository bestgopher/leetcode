#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let left = Self::remove_leaf_nodes(
            root.as_ref()
                .map_or_else(|| None, |x| x.borrow_mut().left.take()),
            target,
        );
        let right = Self::remove_leaf_nodes(
            root.as_ref()
                .map_or_else(|| None, |x| x.borrow_mut().right.take()),
            target,
        );

        if left.is_none() && right.is_none() && root.as_ref().unwrap().borrow().val == target {
            return None;
        }

        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;

        root
    }
}
