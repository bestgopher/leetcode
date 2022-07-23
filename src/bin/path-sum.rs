#![allow(dead_code, unused, unused_variables)]

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(r) = root {
            let value = Rc::clone(&r).borrow().val;
            let left = Rc::clone(&r).borrow_mut().left.take();
            let right = Rc::clone(&r).borrow_mut().right.take();

            if left.is_none() && right.is_none() && value == target_sum {
                return true;
            }

            return Self::has_path_sum(left, target_sum - value)
                || Self::has_path_sum(right, target_sum - value);
        }
        false
    }
}
