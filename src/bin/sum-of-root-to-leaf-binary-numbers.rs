#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {}

struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::f(root, 0)
    }

    fn f(root: Option<Rc<RefCell<TreeNode>>>, i: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        let new_i = (i << 1) + root.borrow().val;

        match (left, right) {
            (None, None) => new_i,
            (Some(l), Some(y)) => Self::f(Some(l), new_i) + Self::f(Some(y), new_i),
            (Some(l), None) => Self::f(Some(l), new_i),
            (None, Some(y)) => Self::f(Some(y), new_i),
        }
    }
}
